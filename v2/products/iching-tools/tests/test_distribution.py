"""Fresh wheel/sdist and clean-install tests for the distributable surface."""

import os
import json
import subprocess
import sys
import tarfile
import venv
import zipfile
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parent.parent
PROBE = ROOT / "mcp" / "tests" / "sdk_probe.py"


@pytest.fixture(scope="module")
def fresh_artifacts(tmp_path_factory):
    output = tmp_path_factory.mktemp("iching-dist")
    result = subprocess.run(
        [sys.executable, "-m", "build", "--wheel", "--sdist", "--outdir", str(output)],
        cwd=ROOT,
        capture_output=True,
        text=True,
        timeout=180,
    )
    assert result.returncode == 0, result.stdout + result.stderr
    wheels = sorted(output.glob("*.whl"))
    sdists = sorted(output.glob("*.tar.gz"))
    assert len(wheels) == 1
    assert len(sdists) == 1
    return wheels[0], sdists[0]


def test_fresh_wheel_contains_all_packages_and_mcp_entrypoint(fresh_artifacts):
    wheel, _ = fresh_artifacts
    with zipfile.ZipFile(wheel) as archive:
        names = set(archive.namelist())
        for package in (
            "iching_tools",
            "iching_coverage",
            "iching_reframe",
            "iching_statediff",
            "iching_cl3calc",
            "iching_xai",
            "iching_rotor",
            "iching_mcp",
        ):
            assert f"{package}/__init__.py" in names
        for path in (
            "iching_mcp/__init__.py",
            "iching_mcp/__main__.py",
            "iching_mcp/contracts.py",
            "iching_mcp/server.py",
        ):
            assert path in names
        assert not any(name.startswith("mcp/") for name in names)

        metadata = next(
            archive.read(name).decode("utf-8")
            for name in names
            if name.endswith("/METADATA")
        )
        entry_points = next(
            archive.read(name).decode("utf-8")
            for name in names
            if name.endswith("/entry_points.txt")
        )

    assert "Name: iching-tools" in metadata
    assert "Version: 0.2.0" in metadata
    assert "License: MIT" in metadata
    assert "iching-mcp = iching_mcp.server:main" in entry_points


def test_fresh_sdist_contains_readme_and_license_metadata(fresh_artifacts):
    _, sdist = fresh_artifacts
    with tarfile.open(sdist) as archive:
        names = set(archive.getnames())
        assert any(name.endswith("/README.md") for name in names)
        assert any(name.endswith("/pyproject.toml") for name in names)
        for package in (
            "iching_tools",
            "iching_coverage",
            "iching_reframe",
            "iching_statediff",
            "iching_cl3calc",
            "iching_xai",
            "iching_rotor",
            "iching_mcp",
        ):
            assert any(name.endswith(f"/{package}/__init__.py") for name in names)
        assert any(name.endswith("/iching_mcp/__main__.py") for name in names)
        assert any(name.endswith("/iching_mcp/contracts.py") for name in names)
        pkg_info = next(
            archive.extractfile(name).read().decode("utf-8")
            for name in names
            if name.endswith("/PKG-INFO")
        )
    assert "Name: iching-tools" in pkg_info
    assert "Version: 0.2.0" in pkg_info
    assert "License: MIT" in pkg_info


def test_wheel_clean_install_has_no_repository_import_dependency(fresh_artifacts, tmp_path):
    wheel, _ = fresh_artifacts
    target = tmp_path / "install"
    target.mkdir()
    env = dict(os.environ)
    env.pop("PYTHONPATH", None)
    result = subprocess.run(
        [sys.executable, "-m", "pip", "install", "--no-deps", "--target", str(target), str(wheel)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        timeout=180,
        env=env,
    )
    assert result.returncode == 0, result.stdout + result.stderr

    env["PYTHONPATH"] = str(target)
    probe = subprocess.run(
        [
            sys.executable,
            "-S",
            "-c",
            "import iching_mcp, iching_mcp.server, iching_cl3calc, iching_xai, iching_rotor; print(iching_mcp.__version__)",
        ],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        timeout=60,
        env=env,
    )
    assert probe.returncode == 0, probe.stdout + probe.stderr
    assert probe.stdout.strip() == "0.2.0"


def test_installed_wheel_console_entrypoint_runs_without_repository_paths(
    fresh_artifacts, tmp_path
):
    wheel, _ = fresh_artifacts
    environment = tmp_path / "venv"
    created = subprocess.run(
        [sys.executable, "-m", "venv", str(environment)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        timeout=180,
    )
    assert created.returncode == 0, created.stdout + created.stderr

    python = environment / ("Scripts" if os.name == "nt" else "bin") / (
        "python.exe" if os.name == "nt" else "python"
    )
    installed = subprocess.run(
        [str(python), "-m", "pip", "install", "--no-deps", str(wheel)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        timeout=180,
    )
    assert installed.returncode == 0, installed.stdout + installed.stderr

    scripts = environment / ("Scripts" if os.name == "nt" else "bin")
    entrypoint = scripts / ("iching-mcp.exe" if os.name == "nt" else "iching-mcp")
    assert entrypoint.exists()
    env = dict(os.environ)
    env.pop("PYTHONPATH", None)
    request = json.dumps({"jsonrpc": "2.0", "id": 1, "method": "tools/list"}) + "\n"
    result = subprocess.run(
        [str(entrypoint), "--sim"],
        input=request,
        cwd=tmp_path,
        capture_output=True,
        text=True,
        timeout=60,
        env=env,
    )
    assert result.returncode == 0, result.stdout + result.stderr
    response = json.loads(result.stdout)
    assert len(response["result"]["tools"]) == 6


def test_official_sdk_calls_all_six_from_clean_install(fresh_artifacts, tmp_path):
    wheel, _ = fresh_artifacts
    target = tmp_path / "install"
    target.mkdir()
    env = dict(os.environ)
    env.pop("PYTHONPATH", None)
    install = subprocess.run(
        [sys.executable, "-m", "pip", "install", "--no-deps", "--target", str(target), str(wheel)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        timeout=180,
        env=env,
    )
    assert install.returncode == 0, install.stdout + install.stderr

    env["PYTHONPATH"] = str(target)
    probe = subprocess.run(
        [sys.executable, str(PROBE), "--installed", str(tmp_path)],
        cwd=tmp_path,
        capture_output=True,
        text=True,
        timeout=180,
        env=env,
    )
    assert probe.returncode == 0, probe.stdout[-2000:] + probe.stderr[-4000:]
    assert "CALLS:" in probe.stdout
