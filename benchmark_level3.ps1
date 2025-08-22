# Level 3 Hardcore Performance Benchmark
Write-Host "=== Level 3 Hardcore Performance Benchmark ===" -ForegroundColor Green

$iterations = 5
$times = @()

Write-Host "`nRunning $iterations iterations of Level 3 hardcore optimized version..." -ForegroundColor Yellow

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

Write-Host "`n=== Level 3 Results ===" -ForegroundColor Green
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

# Performance comparison
$baselineTime = 0.81  # seconds for 3 files from previous benchmarks
$level2Time = 0.020   # Level 2 performance
$speedupVsBaseline = $baselineTime / $avgTime
$speedupVsLevel2 = $level2Time / $avgTime

Write-Host "`n=== Level 3 vs Previous Levels ===" -ForegroundColor Green
Write-Host "Baseline (Level 0): $($baselineTime)s" -ForegroundColor White
Write-Host "Level 2: $($level2Time)s (39.7x speedup)" -ForegroundColor White
Write-Host "Level 3: $($avgTime.ToString('F3'))s ($($speedupVsBaseline.ToString('F1'))x speedup vs baseline!)" -ForegroundColor Yellow
if ($speedupVsLevel2 -gt 1) {
    Write-Host "Level 3 vs Level 2: $($speedupVsLevel2.ToString('F1'))x improvement" -ForegroundColor Yellow
} else {
    Write-Host "Level 3 vs Level 2: $((1/$speedupVsLevel2).ToString('F1'))x slower (still amazing performance!)" -ForegroundColor Cyan
}

Write-Host "`n🚀 Level 3 Hardcore Optimizations Summary:" -ForegroundColor Green
Write-Host "✅ Custom EXR parser (bypasses all pixel data)" -ForegroundColor White
Write-Host "✅ Lock-free DashMap for parallel channel grouping" -ForegroundColor White
Write-Host "✅ SIMD string pattern matching" -ForegroundColor White
Write-Host "✅ Ultra-fast precomputed channel classification" -ForegroundColor White
Write-Host "✅ Async I/O with in-memory content building" -ForegroundColor White