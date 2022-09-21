#!/usr/bin/fish

set ns (seq 128)

set n[1] 45
for i in $ns
    echo $i
    set ni (math $i + 1)
    set q (tungsten "decimal approximation of atan(2^[-$i]) * 180 / PI")
    set n[$ni] $q[7]
end

for i in 0 $ns
    set ni (math $i + 1)
    echo $n[$ni]
end
