LIBRARY IEEE;
USE ieee.std_logic_1164.all;

USE work.package_1_pkg.ALL;
--USE work.pack_does_not_exist_pkg.ALL;

ARCHITECTURE rtl OF design_1 IS
BEGIN

    z <= NOT a AFTER 10 ns;
    
END ARCHITECTURE rtl;
