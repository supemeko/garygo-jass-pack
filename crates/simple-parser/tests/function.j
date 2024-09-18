globals
    constant integer b = 20
    constant integer a = 5 + 10 * b + 15
endglobals

function Add takes integer i, integer j returns integer
    return i + j
endfunction

function SetUnitLife takes integer i returns nothing
    return
endfunction

function Main takes nothing returns nothing
    local integer l
    set l = Add(10, 5)
endfunction
