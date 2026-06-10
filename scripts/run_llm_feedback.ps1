param(
    [string]$ApiKey = "",
    [string]$InputFile = "data/llm_feedback_prompts.json",
    [string]$OutputFile = "data/llm_feedback_results.json",
    [int]$MaxPairs = 100
)

$ErrorActionPreference = "Stop"

if ($ApiKey -eq "") {
    $envPath = Join-Path (Join-Path $PSScriptRoot "..") ".env.local"
    if (Test-Path $envPath) {
        Get-Content $envPath | ForEach-Object {
            if ($_ -match "^OPENROUTER_API_KEY=(.+)$") { $ApiKey = $matches[1] }
        }
    }
}

if ($ApiKey -eq "") {
    Write-Error "No API key. Set OPENROUTER_API_KEY in .env.local or pass -ApiKey"
    exit 1
}

$inputPath = Join-Path (Join-Path $PSScriptRoot "..") $InputFile
if (-not (Test-Path $inputPath)) {
    Write-Error "Input file not found: $inputPath. Run generate_feedback_prompts test first."
    exit 1
}

$data = Get-Content $inputPath -Raw | ConvertFrom-Json
$results = @()
$count = [Math]::Min($data.failing_pairs.Count, $MaxPairs)

Write-Host "Processing $count failing pairs via OpenRouter..."

for ($i = 0; $i -lt $count; $i++) {
    $pair = $data.failing_pairs[$i]
    Write-Host "  [$($i+1)/$count] $($pair.concept_a) -> $($pair.concept_b) (expected: $($pair.expected_label))"

    $systemPrompt = @"
You are encoding concepts into 8-element geometric algebra coefficient arrays for a Bagua (I-Ching) semantic knowledge graph.

Each of the 8 indices maps to a specific semantic role:
0: receptive (Kun/Earth) - accepts, follows, grounds
1: causal (Zhen/Wood) - triggers, initiates, starts chains
2: transmissive (Kan/Water) - channels, flows, transmits
3: constraining (Gen/Earth) - limits, bounds, restricts
4: clarifying (Li/Fire) - reveals, illuminates, makes visible
5: influential (Xun/Wood) - pervades, gradually shapes
6: balancing (Dui/Metal) - mirrors, equilibrates, reflects
7: generative (Qian/Metal) - creates, enables, produces

Guidelines:
- Strongly exhibits: 0.5-1.0
- Moderately exhibits: 0.2-0.5
- Slightly exhibits: 0.05-0.2
- Irrelevant: -0.05 to 0.05
- Slightly counters: -0.2 to -0.05
- Moderately counters: -0.5 to -0.2
- Strongly counters: -1.0 to -0.5

For each concept, ask what it DOES (not what it IS). Response must be ONLY valid JSON.
"@

    $userPrompt = @"
$($pair.prompt)

Current encodings:
  $($pair.concept_a): $($pair.a_coefficients -join ', ')
  $($pair.concept_b): $($pair.b_coefficients -join ', ')

Return ONLY a JSON object with the re-encoded coefficients:
{
  "a": [receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative],
  "b": [receptive, causal, transmissive, constraining, clarifying, influential, balancing, generative]
}
Each value must be a float in [-1.0, 1.0]. Do NOT include any other text.
"@

    try {
        $body = @{
            model = "nex-agi/nex-n2-pro:free"
            messages = @(
                @{ role = "system"; content = $systemPrompt },
                @{ role = "user"; content = $userPrompt }
            )
            max_tokens = 500
            temperature = 0.3
        } | ConvertTo-Json -Depth 10

        $response = Invoke-RestMethod -Uri "https://openrouter.ai/api/v1/chat/completions" `
            -Method Post `
            -Headers @{
                "Authorization" = "Bearer $ApiKey"
                "Content-Type" = "application/json"
            } `
            -Body ([System.Text.Encoding]::UTF8.GetBytes($body)) `
            -TimeoutSec 30

        $content = $response.choices[0].message.content
        # Extract JSON from response (strip markdown code fences if present)
        $content = $content -replace '```json\s*','' -replace '```\s*','' -replace '^\s+','' -replace '\s+$',''

        try {
            $parsed = $content | ConvertFrom-Json
            $results += @{
                concept_a = $pair.concept_a
                concept_b = $pair.concept_b
                expected_label = $pair.expected_label
                current_label = $pair.current_label
                new_a = @($parsed.a)
                new_b = @($parsed.b)
                raw_response = $content
            }
            Write-Host "    -> Got re-encoding (a dominant: check JSON)"
        } catch {
            Write-Host "    -> Failed to parse JSON response: $($_.Exception.Message)"
            Write-Host "    -> Raw: $content"
            $results += @{
                concept_a = $pair.concept_a
                concept_b = $pair.concept_b
                expected_label = $pair.expected_label
                current_label = $pair.current_label
                new_a = $null
                new_b = $null
                raw_response = $content
                error = $_.Exception.Message
            }
        }
    } catch {
        Write-Host "    -> API call failed: $($_.Exception.Message)"
        Start-Sleep -Seconds 2
    }
}

$outputPath = Join-Path (Join-Path $PSScriptRoot "..") $OutputFile
$results | ConvertTo-Json -Depth 10 | Out-File -FilePath $outputPath -Encoding UTF8
Write-Host "`nSaved $($results.Count) results to $OutputFile"
Write-Host "Now run: cargo test -p ga-semantics-core --test apply_llm_feedback -- --nocapture"
