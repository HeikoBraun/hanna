LIBRARY IEEE;
USE ieee.std_logic_1164.all;
USE ieee.numeric_std.all;

LIBRARY lib_2;

ARCHITECTURE structure OF design_1 IS
    SIGNAL sig : unsigned(a'range);
BEGIN

    inst_i1 : entity lib_2.design_1(rtl)
    PORT MAP(
        a => a,
        z => sig
    );

    inst_i2 : configuration lib_2.cfg_design_2
    PORT MAP(
        a => sig,
        z => z
    );
    
END ARCHITECTURE structure;
