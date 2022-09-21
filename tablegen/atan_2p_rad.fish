#!/usr/bin/fish

set ns 0 (seq 128)

for i in $ns
    echo $i
    set ni (math $i + 1)
    set q (tungsten "decimal approximation of atan(2^[-$i])")
    set n[$ni] $q[7]
end

for i in $ns
    set ni (math $i + 1)
    echo $n[$ni]
end
