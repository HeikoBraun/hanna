LIBRARY IEEE;
USE ieee.std_logic_1164.all;
USE ieee.numeric_std.all;

ENTITY design_2 IS
    PORT(
        a: IN    unsigned(3 DOWNTO 0);
        z:   OUT unsigned(3 DOWNTO 0)
    );
END ENTITY design_2;

ARCHITECTURE rtl OF design_2 IS
BEGIN

    z <= a + 2;

END ARCHITECTURE rtl;

CONFIGURATION cfg_design_2 OF design_2 IS
    FOR rtl
    END FOR;
END CONFIGURATION cfg_design_2;
