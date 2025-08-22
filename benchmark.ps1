# Level 2 Performance Benchmark
Write-Host "=== EXR Tools Level 2 Performance Benchmark ===" -ForegroundColor Green

$iterations = 5
$times = @()

Write-Host "`nRunning $iterations iterations of Level 2 optimized version..." -ForegroundColor Yellow

for ($i = 1; $i -le $iterations; $i++) {
    Write-Host "Iteration $i of $iterations..." -ForegroundColor Cyan
    
    # Clean up previous outputs
    Remove-Item *.txt -ErrorAction SilentlyContinue
    
    # Measure execution time
    $startTime = Get-Date
    & .\target\release\readEXR.exe | Out-Null
    $endTime = Get-Date
    
    $duration = ($endTime - $startTime).TotalSeconds
    $times += $duration
    Write-Host "  Time: $($duration.ToString('F3'))s" -ForegroundColor White
}

# Calculate statistics
$avgTime = ($times | Measure-Object -Average).Average
$minTime = ($times | Measure-Object -Minimum).Minimum
$maxTime = ($times | Measure-Object -Maximum).Maximum

Write-Host "`n=== Level 2 Results ===" -ForegroundColor Green
Write-Host "Average time: $($avgTime.ToString('F3'))s" -ForegroundColor White
Write-Host "Min time: $($minTime.ToString('F3'))s" -ForegroundColor White
Write-Host "Max time: $($maxTime.ToString('F3'))s" -ForegroundColor White

# File size analysis
Write-Host "`n=== File Analysis ===" -ForegroundColor Green
$exrFiles = Get-ChildItem data\*.exr
$totalSize = ($exrFiles | Measure-Object -Property Length -Sum).Sum / 1MB

Write-Host "Files processed: $($exrFiles.Count)" -ForegroundColor White
Write-Host "Total size: $($totalSize.ToString('F1'))MB" -ForegroundColor White
Write-Host "Throughput: $($($totalSize / $avgTime).ToString('F1'))MB/s" -ForegroundColor White

# Performance compared to baseline from opt.md
$baselineTime = 1.08  # seconds for 4 files from opt.md
$baselineFiles = 4
$currentFiles = $exrFiles.Count

# Normalize for file count difference
$normalizedBaseline = $baselineTime * ($currentFiles / $baselineFiles)
$speedup = $normalizedBaseline / $avgTime

Write-Host "`n=== Performance Improvement ===" -ForegroundColor Green
Write-Host "Baseline (Level 0): $($normalizedBaseline.ToString('F3'))s for $currentFiles files" -ForegroundColor White
Write-Host "Level 2 Optimized: $($avgTime.ToString('F3'))s for $currentFiles files" -ForegroundColor White
Write-Host "Speedup: $($speedup.ToString('F1'))x faster!" -ForegroundColor Yellow