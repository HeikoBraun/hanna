LIBRARY IEEE;
USE ieee.std_logic_1164.all;
USE ieee.numeric_std.all;

ARCHITECTURE struct OF design_1 IS
    SIGNAL sig : unsigned(a'range);
    COMPONENT des IS
        PORT(
            a: IN    unsigned(3 DOWNTO 0);
            z:   OUT unsigned(3 DOWNTO 0)
        );
    END COMPONENT;
BEGIN

    inst_i1 : des
    PORT MAP(
        a => a,
        z => sig
    );

    inst_i2 : des
    PORT MAP(
        a => sig,
        z => z
    );
    
END ARCHITECTURE struct;
