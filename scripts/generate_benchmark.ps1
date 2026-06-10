# Generate ga-bagua benchmark dataset with 50 concepts, 50 relations, train-test split
$ErrorActionPreference = "Stop"
$dataDir = Join-Path (Join-Path $PSScriptRoot "..") "data"
New-Item -ItemType Directory -Path $dataDir -Force | Out-Null

function Normalize-Coeffs($raw) {
    $ss = 0.0; foreach ($v in $raw) { $ss += $v * $v }
    $norm = [Math]::Sqrt($ss)
    if ($norm -lt 1e-12) { return @(1.0, 0, 0, 0, 0, 0, 0, 0) }
    $r = @(); foreach ($v in $raw) { $r += [Math]::Round($v / $norm, 4) }
    return $r
}

# ── CONCEPT DEFINITIONS ──
# [name, description, domain, [8 raw coefficients]]
$conceptDefs = @(
    # === SOFTWARE ARCHITECTURE (0-16) ===
    @("Rate Limiter", "Restricts the number of requests a client can make in a time window", "software_architecture", @(0.05, -0.10, -0.60, 0.80, 0.25, -0.30, 0.20, -0.40)),
    @("Message Queue", "Transmits events between services asynchronously with guaranteed delivery", "software_architecture", @(0.15, 0.25, 0.80, -0.20, -0.25, 0.10, 0.35, 0.05)),
    @("Database Index", "Data structure that accelerates record lookup by organizing keys for fast retrieval", "software_architecture", @(0.10, 0.10, 0.30, 0.05, 0.80, 0.05, 0.10, 0.10)),
    @("API Gateway", "Routes incoming client requests to appropriate backend services", "software_architecture", @(0.20, 0.30, 0.75, 0.05, -0.10, 0.15, 0.25, 0.05)),
    @("Load Balancer", "Distributes traffic evenly across multiple server instances", "software_architecture", @(0.15, -0.05, 0.30, -0.05, 0.10, 0.10, 0.80, 0.05)),
    @("Circuit Breaker", "Prevents cascading failures by stopping calls to failing services", "software_architecture", @(0.05, -0.25, -0.20, 0.78, 0.25, -0.10, 0.20, -0.15)),
    @("Monitoring Dashboard", "Observes system health metrics and alerts on anomalies", "software_architecture", @(0.20, 0.25, 0.05, 0.10, 0.80, 0.10, 0.25, -0.05)),
    @("Feature Flag", "Toggle that enables gradual rollout of new functionality to user subsets", "software_architecture", @(0.10, 0.20, 0.15, -0.10, 0.10, 0.78, 0.30, 0.35)),
    @("Logging System", "Records every operation for audit trails and debugging", "software_architecture", @(0.15, 0.05, 0.10, 0.20, 0.85, 0.05, 0.25, -0.10)),
    @("Configuration Store", "Central repository for application configuration values", "software_architecture", @(0.70, 0.05, 0.10, 0.15, 0.30, 0.15, 0.10, 0.00)),
    @("Authentication Provider", "Verifies identity before granting access to protected resources", "software_architecture", @(0.25, 0.15, -0.10, 0.65, 0.40, 0.05, 0.25, 0.20)),
    @("Cache Layer", "Stores frequently accessed data in memory for fast retrieval", "software_architecture", @(0.30, 0.10, 0.65, -0.20, -0.30, 0.15, 0.30, 0.10)),
    @("Event Stream Processor", "Transforms and enriches data in real-time event pipelines", "software_architecture", @(0.10, 0.30, 0.65, -0.10, 0.15, 0.25, 0.15, 0.40)),
    @("Database Transaction", "Ensures atomicity consistency isolation and durability across writes", "software_architecture", @(0.30, 0.05, 0.15, 0.80, 0.35, 0.18, 0.37, 0.09)),
    @("Deprecation Policy", "Planned phase-out that gradually shapes migration away from old APIs", "software_architecture", @(0.20, 0.10, 0.05, 0.25, 0.15, 0.78, 0.15, 0.10)),
    @("Health Check Endpoint", "Diagnostic API that reveals whether a service is functioning correctly", "software_architecture", @(0.15, 0.10, 0.10, 0.10, 0.80, 0.10, 0.15, 0.05)),
    @("Schema Registry", "Enforces data format rules across all producers and consumers", "software_architecture", @(0.15, 0.05, 0.15, 0.78, 0.30, 0.10, 0.10, 0.05)),

    # === BUSINESS OPERATIONS (17-33) ===
    @("Marketing Budget", "Financial allocation that caps promotional spending per quarter", "business_operations", @(0.05, 0.05, 0.10, 0.82, -0.05, 0.25, 0.10, 0.10)),
    @("Sales Pipeline", "Staged funnel through which leads progress from contact to close", "business_operations", @(0.10, 0.15, 0.78, -0.05, -0.10, 0.15, 0.20, 0.20)),
    @("Customer Support Ticket", "User-reported issue that initiates a diagnostic and resolution workflow", "business_operations", @(0.10, 0.75, 0.20, 0.05, 0.10, 0.25, 0.10, 0.15)),
    @("Quarterly Report", "Structured document revealing financial performance to stakeholders", "business_operations", @(0.15, 0.10, 0.10, 0.10, 0.80, 0.10, 0.10, 0.10)),
    @("Revenue Target", "Ambitious income goal that motivates organizational effort", "business_operations", @(0.10, 0.30, 0.10, -0.10, 0.15, 0.10, 0.15, 0.82)),
    @("Employee Handbook", "Comprehensive policy document defining acceptable workplace conduct", "business_operations", @(0.15, 0.05, 0.05, 0.80, 0.30, 0.25, 0.10, 0.05)),
    @("Vendor Contract", "Binding legal agreement that obligates both parties to specific terms", "business_operations", @(0.10, 0.10, 0.05, 0.80, 0.15, 0.20, 0.15, 0.05)),
    @("Innovation Fund", "Ring-fenced capital pool that enables experimental projects to launch", "business_operations", @(0.05, 0.25, 0.15, -0.15, 0.10, 0.15, 0.10, 0.82)),
    @("Customer Feedback Loop", "Systematic process for collecting analyzing and responding to user input", "business_operations", @(0.25, 0.10, 0.25, 0.05, 0.15, 0.15, 0.78, 0.10)),
    @("Onboarding Program", "Structured workflow that channels new hires from orientation to productivity", "business_operations", @(0.20, 0.15, 0.75, 0.05, 0.15, 0.10, 0.25, 0.15)),
    @("Market Trend Analysis", "Research that shapes strategic direction through accumulated insight", "business_operations", @(0.15, 0.15, 0.10, 0.05, 0.80, 0.15, 0.10, 0.15)),
    @("Compliance Audit", "Formal examination that reveals regulatory adherence or gaps", "business_operations", @(0.10, 0.10, 0.10, 0.30, 0.80, 0.10, 0.15, 0.10)),
    @("Severance Agreement", "Structured compensation that accepts the termination of employment", "business_operations", @(0.78, 0.05, 0.15, 0.15, 0.10, 0.10, 0.20, 0.10)),
    @("Industry Regulation", "Widely adopted standard that pervasively shapes how companies operate", "business_operations", @(0.20, 0.10, 0.15, 0.20, 0.10, 0.78, 0.20, 0.05)),
    @("Team Standup Meeting", "Daily sync meeting that mirrors progress and equilibrates team awareness", "business_operations", @(0.15, 0.15, 0.15, 0.05, 0.15, 0.20, 0.80, 0.10)),
    @("Supply Chain", "Network that channels goods from raw materials to end customers", "business_operations", @(0.15, 0.20, 0.78, 0.05, 0.10, 0.15, 0.20, 0.15)),
    @("Hiring Freeze", "Organizational mandate that constrains workforce expansion", "business_operations", @(0.05, -0.30, -0.15, 0.75, 0.20, -0.10, 0.25, -0.25)),

    # === BIOLOGICAL SYSTEMS (34-49) ===
    @("Predator", "Animal that limits prey population through hunting and consumption", "biological_systems", @(0.05, 0.15, 0.10, 0.80, 0.10, 0.15, 0.15, 0.15)),
    @("Decomposer", "Organism that breaks down dead matter and recycles it into soil nutrients", "biological_systems", @(0.78, 0.05, 0.20, 0.10, 0.15, 0.10, 0.20, 0.05)),
    @("Photosynthesis", "Biological process that creates energy-rich compounds from sunlight", "biological_systems", @(0.05, 0.20, 0.15, 0.05, 0.10, 0.15, 0.10, 0.82)),
    @("Water Cycle", "Continuous movement channeling water between atmosphere land and oceans", "biological_systems", @(0.15, 0.20, 0.78, 0.05, 0.10, 0.15, 0.20, 0.15)),
    @("Keystone Species", "Species whose presence pervasively shapes the entire ecosystem structure", "biological_systems", @(0.10, 0.25, 0.15, 0.10, 0.10, 0.78, 0.20, 0.25)),
    @("Mutation", "Random genetic change that triggers variation within a population", "biological_systems", @(0.10, 0.78, 0.15, 0.05, 0.15, 0.10, 0.15, 0.20)),
    @("Homeostasis", "Self-regulating process that maintains equilibrium in living systems", "biological_systems", @(0.15, 0.05, 0.15, 0.15, 0.15, 0.10, 0.80, 0.10)),
    @("Natural Selection", "Environmental pressure that constrains which traits propagate", "biological_systems", @(0.05, 0.10, 0.10, 0.80, 0.15, 0.25, 0.15, 0.10)),
    @("Ecological Succession", "Gradual process through which an ecosystem transforms over time", "biological_systems", @(0.15, 0.15, 0.10, 0.05, 0.10, 0.80, 0.20, 0.15)),
    @("Symbiosis", "Close interaction where different species mutually benefit and balance each other", "biological_systems", @(0.20, 0.10, 0.15, 0.05, 0.15, 0.10, 0.78, 0.20)),
    @("DNA Replication", "Molecular process that creates identical copies of genetic material", "biological_systems", @(0.10, 0.20, 0.15, 0.05, 0.10, 0.10, 0.15, 0.78)),
    @("Immune Response", "Defensive cascade that triggers when the body detects foreign pathogens", "biological_systems", @(0.10, 0.75, 0.20, 0.15, 0.10, 0.10, 0.20, 0.10)),
    @("Enzyme Catalyst", "Protein that accelerates biochemical reactions without being consumed", "biological_systems", @(0.10, 0.30, 0.20, 0.05, 0.10, 0.10, 0.10, 0.78)),
    @("Hormone Signal", "Chemical messenger that transmits regulatory information through the body", "biological_systems", @(0.15, 0.30, 0.72, 0.05, 0.10, 0.20, 0.15, 0.10)),
    @("Cell Membrane", "Semi-permeable boundary that bounds cellular contents", "biological_systems", @(0.10, 0.05, 0.10, 0.80, 0.10, 0.15, 0.15, 0.05)),
    @("Neural Plasticity", "Brain ability to gradually reshape connections in response to experience", "biological_systems", @(0.10, 0.20, 0.10, 0.05, 0.10, 0.78, 0.15, 0.25))
)

# ── BUILD CONCEPTS JSON ──
$conceptsJson = @()
for ($i = 0; $i -lt $conceptDefs.Count; $i++) {
    $def = $conceptDefs[$i]
    $norm = Normalize-Coeffs $def[3]
    $conceptsJson += @{
        index = $i
        name = $def[0]
        description = $def[1]
        domain = $def[2]
        coefficients = $norm
    }
}

# ── RELATIONSHIP PAIRS ──
# [idx_a, idx_b, label, confidence]
# Semantic labels based on real-world concept interactions
$relDefs = @(
    # ── INTRA-DOMAIN: Software Architecture ──
    # Rate Limiter constrains API Gateway (throttles requests going through gateway)
    @(0, 3, "constraining", "certain"),
    # Message Queue channels to Event Stream Processor (queue feeds processor)
    @(1, 12, "transmissive", "certain"),
    # Database Index reveals Cache Layer hits (index clarifies what is cached)
    @(2, 11, "clarifying", "plausible"),
    # API Gateway channels to Load Balancer (gateway routes to balanced pool)
    @(3, 4, "transmissive", "certain"),
    # Load Balancer equilibrates Server Load (definitional balancing)
    @(4, 12, "balancing", "certain"),
    # Circuit Breaker constrains Event Stream (stops calls when failing)
    @(5, 12, "constraining", "certain"),
    # Monitoring Dashboard reveals System Health (definitional clarifying)
    @(6, 15, "clarifying", "certain"),
    # Feature Flag gradually shapes Deprecation Policy (flags influence rollout)
    @(7, 14, "influential", "certain"),
    # Logging System receives Health Check data (logs accept health events)
    @(8, 15, "receptive", "certain"),
    # Configuration Store receives deprecation policy configs
    @(9, 14, "receptive", "plausible"),
    # Authentication Provider constrains API Gateway access
    @(10, 3, "constraining", "certain"),
    # Cache Layer speeds up Database Index (cache makes fast retrieval faster)
    @(11, 2, "influential", "plausible"),
    # Event Stream Processor triggers Monitoring alerts (processing reveals issues)
    @(12, 6, "causal", "certain"),
    # Database Transaction receives Load Balancer routing decisions
    @(13, 4, "receptive", "certain"),
    # Schema Registry constrains Event Stream format
    @(16, 12, "constraining", "certain"),

    # ── INTRA-DOMAIN: Business Operations ──
    # Marketing Budget constrains Innovation Fund (budget caps spend)
    @(17, 24, "constraining", "certain"),
    # Sales Pipeline generates Revenue (pipeline produces revenue)
    @(18, 21, "generative", "certain"),
    # Customer Support Ticket triggers Quarterly Report analysis (tickets cause reporting)
    @(19, 20, "causal", "certain"),
    # Quarterly Report clarifies Revenue Target progress
    @(20, 21, "clarifying", "certain"),
    # Employee Handbook constrains Hiring Freeze terms
    @(22, 33, "constraining", "certain"),
    # Vendor Contract constrains Supply Chain operations
    @(23, 32, "constraining", "certain"),
    # Innovation Fund generates new Revenue streams
    @(24, 21, "generative", "certain"),
    # Customer Feedback Loop equilibrates Market Trend Analysis (feedback balances trends)
    @(25, 27, "balancing", "certain"),
    # Onboarding Program channels new hires toward Innovation Fund projects
    @(26, 24, "transmissive", "plausible"),
    # Market Trend Analysis gradually shapes Industry Regulation standards
    @(27, 30, "influential", "certain"),
    # Compliance Audit reveals Marketing Budget adherence
    @(28, 17, "clarifying", "certain"),
    # Severance Agreement receives Hiring Freeze decisions (severance accepts terminations)
    @(29, 33, "receptive", "certain"),
    # Industry Regulation gradually shapes Employee Handbook policies
    @(30, 22, "influential", "certain"),
    # Team Standup Meeting equilibrates Customer Feedback awareness (sync balances team knowledge)
    @(31, 25, "balancing", "certain"),
    # Supply Chain channels toward Revenue generation (goods flow generates revenue)
    @(32, 21, "transmissive", "certain"),

    # ── INTRA-DOMAIN: Biological Systems ──
    # Predator constrains Prey population (definitional constraining)
    @(34, 39, "constraining", "plausible"),
    # Decomposer receives dead matter from Photosynthesis products
    @(35, 36, "receptive", "certain"),
    # Photosynthesis generates energy for Homeostasis
    @(36, 40, "generative", "certain"),
    # Water Cycle channels water to Photosynthesis (water enables photosynthesis)
    @(37, 36, "transmissive", "certain"),
    # Keystone Species pervasively shapes Ecological Succession
    @(38, 42, "influential", "certain"),
    # Mutation triggers variation for Natural Selection (mutation provides raw material)
    @(39, 41, "causal", "certain"),
    # Homeostasis equilibrates Symbiosis relationships (balance maintains mutualism)
    @(40, 43, "balancing", "certain"),
    # Natural Selection constrains which Mutations survive (selection limits)
    @(41, 39, "constraining", "certain"),
    # Ecological Succession gradually shapes Decomposer communities
    @(42, 35, "influential", "certain"),
    # Symbiosis equilibrates Keystone Species relationships
    @(43, 38, "balancing", "certain"),
    # DNA Replication generates material for Cellular Homeostasis
    @(44, 40, "generative", "certain"),
    # Immune Response triggers Hormone Signaling cascade
    @(45, 47, "causal", "certain"),
    # Enzyme Catalyst accelerates DNA Replication (enzyme speeds up DNA copying)
    @(46, 44, "generative", "certain"),
    # Hormone Signal transmits regulatory info to Cell Membrane receptors
    @(47, 48, "transmissive", "certain"),
    # Cell Membrane constrains what enters the cell
    @(48, 44, "constraining", "plausible"),

    # ── CROSS-DOMAIN RELATIONSHIPS (8 pairs) ──
    # Rate Limiter (SW) constrains Customer Support Ticket (Biz) flow (limits incoming tickets)
    @(0, 19, "constraining", "certain"),
    # Mutation (Bio) triggers Innovation Fund (Biz) creative thinking
    @(39, 24, "causal", "plausible"),
    # Load Balancer (SW) mirrors Homeostasis (Bio) pattern (both equilibrate systems)
    @(4, 40, "balancing", "certain"),
    # Industry Regulation (Biz) constrains Schema Registry (SW) data rules
    @(30, 16, "constraining", "certain"),
    # Water Cycle (Bio) channels metaphor for Supply Chain (Biz) flow
    @(37, 32, "transmissive", "plausible"),
    # Keystone Species (Bio) influences Industry Regulation (Biz) thinking on standards
    @(38, 30, "influential", "plausible"),
    # Compliance Audit (Biz) reveals Circuit Breaker (SW) status
    @(28, 5, "clarifying", "certain"),
    # DNA Replication (Bio) analogy generates Event Stream Processor (SW) design patterns
    @(44, 12, "generative", "plausible")
)

# ── BUILD RELATIONS JSON ──
$relationsJson = @()
for ($i = 0; $i -lt $relDefs.Count; $i++) {
    $def = $relDefs[$i]
    $relationsJson += @{
        index = $i
        idx_a = $def[0]
        idx_b = $def[1]
        label = $def[2]
        confidence = $def[3]
        cross_domain = ($conceptsJson[$def[0]].domain -ne $conceptsJson[$def[1]].domain)
    }
}

# ── TRAIN-TEST SPLIT ──
# Stratified: domain-proportional representation in both splits
# SW (0-16): train 0-11 (12), test 12-16 (5)
# Biz (17-33): train 17-28 (12), test 29-33 (5)
# Bio (34-49): train 34-44 (11), test 45-49 (5)
$trainConceptIndices = @(0..11) + @(17..28) + @(34..44)
$testConceptIndices = @(12..16) + @(29..33) + @(45..49)

$trainRelations = @()
$testRelations = @()
for ($i = 0; $i -lt $relationsJson.Count; $i++) {
    $r = $relationsJson[$i]
    $inTrain = ($trainConceptIndices -contains $r.idx_a) -and ($trainConceptIndices -contains $r.idx_b)
    if ($inTrain) {
        $trainRelations += $i
    } else {
        $testRelations += $i
    }
}

# ── VALIDATION ──
$labelCounts = @{}
foreach ($r in $relDefs) { $labelCounts[$r[2]] = ($labelCounts[$r[2]] -or 0) + 1 }
Write-Host "Relation labels:"
foreach ($k in ($labelCounts.Keys | Sort-Object)) { Write-Host "  $k`: $($labelCounts[$k])" }
$crossDomainCount = ($relationsJson | Where-Object { $_.cross_domain }).Count
Write-Host "Cross-domain pairs: $crossDomainCount"
Write-Host "Train concepts: $($trainConceptIndices.Count), Test concepts: $($testConceptIndices.Count)"
Write-Host "Train relations: $($trainRelations.Count), Test relations: $($testRelations.Count)"

# Verify all 8 labels appear at least twice
foreach ($label in @("generative","receptive","causal","transmissive","constraining","influential","clarifying","balancing")) {
    $count = $labelCounts[$label] -or 0
    if ($count -lt 2) { Write-Warning "Label '$label' only appears $count times (need >= 2)" }
}

# ── ASSEMBLE AND WRITE JSON ──
$dataset = @{
    meta = @{
        description = "GA-Bagua semantic knowledge graph benchmark dataset"
        generated_at = (Get-Date -Format "o")
        num_concepts = $conceptsJson.Count
        num_relations = $relationsJson.Count
        domains = @("software_architecture", "business_operations", "biological_systems")
        domain_counts = @{
            software_architecture = ($conceptsJson | Where-Object { $_.domain -eq "software_architecture" }).Count
            business_operations = ($conceptsJson | Where-Object { $_.domain -eq "business_operations" }).Count
            biological_systems = ($conceptsJson | Where-Object { $_.domain -eq "biological_systems" }).Count
        }
        encoding_protocol = "8-role Bagua semantic encoding with unit L2 norm"
        semantic_roles = @("receptive", "causal", "transmissive", "constraining", "clarifying", "influential", "balancing", "generative")
    }
    concepts = $conceptsJson
    relations = $relationsJson
    split = @{
        strategy = "stratified by domain: SW 12/5, Biz 12/5, Bio 11/5 train/test"
        train_concept_indices = $trainConceptIndices
        test_concept_indices = $testConceptIndices
        train_relation_indices = $trainRelations
        test_relation_indices = $testRelations
        train_concept_count = $trainConceptIndices.Count
        test_concept_count = $testConceptIndices.Count
        train_relation_count = $trainRelations.Count
        test_relation_count = $testRelations.Count
    }
}

$json = ConvertTo-Json -InputObject $dataset -Depth 5 -Compress:$false
$outPath = Join-Path $dataDir "benchmark_dataset.json"
[System.IO.File]::WriteAllText($outPath, $json, [System.Text.UTF8Encoding]::new($false))
Write-Host "Written to: $outPath"
$fileInfo = Get-Item $outPath
Write-Host "Size: $([Math]::Round($fileInfo.Length / 1024, 1)) KB"
