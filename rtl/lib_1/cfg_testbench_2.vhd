LIBRARY IEEE;
USE ieee.std_logic_1164.all;

CONFIGURATION cfg_testbench_2 OF testbench IS
    FOR structure
        FOR duv_i : duv
            USE CONFIGURATION work.cfg_design_2;
        end FOR;
    END FOR;
END CONFIGURATION cfg_testbench_2;
