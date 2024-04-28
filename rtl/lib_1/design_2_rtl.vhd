LIBRARY IEEE;
USE ieee.std_logic_1164.all;



ARCHITECTURE rtl OF design_2 IS
BEGIN

    z <= NOT a AFTER 10 ns;
    
END ARCHITECTURE rtl;
