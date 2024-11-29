set terminal pngcairo size 1080,720
set output "fft.png"
set terminal qt

stats "dump.dat" using 1 nooutput

set xrange [STATS_min:STATS_max]
set style data lines
set grid

set key top right

plot "dump.dat" using 1:2 title "" with lines lw 1

pause mouse close
