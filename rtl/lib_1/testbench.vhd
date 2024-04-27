LIBRARY IEEE;
USE ieee.std_logic_1164.all;
USE ieee.numeric_std.all;

ENTITY testbench IS
END ENTITY testbench;

ARCHITECTURE structure OF testbench IS
    CONSTANT Tper : time := 10 ns;
    SIGNAL clk : std_ulogic;
    SIGNAL reset_n : std_ulogic;
    SIGNAL a : unsigned(3 DOWNTO 0);
    SIGNAL z : unsigned(3 DOWNTO 0);

    COMPONENT duv
        PORT(
            a: IN    unsigned(3 DOWNTO 0);
            z:   OUT unsigned(3 DOWNTO 0)
        );
    END COMPONENT;

BEGIN

    reset_proc : PROCESS IS
    BEGIN
        reset_n <= '0';
        WAIT FOR 4*Tper;
        WAIT UNTIL falling_edge(clk);
        reset_n <= '1';
        WAIT;
    END PROCESS;

    clk_proc : PROCESS IS
    BEGIN
        clk <= '0';
        FOR n IN 0 TO 20 LOOP
            WAIT FOR Tper/2;
            clk <= NOT clk;            
        END LOOP;
        REPORT "Simulation finished!";
        WAIT;
    END PROCESS;

    data_proc : PROCESS (clk, reset_n) IS
    BEGIN
        IF reset_n = '0' THEN
            a <= TO_UNSIGNED(0, a'length);
        ELSIF rising_edge(clk) THEN
            a <= a + 1;
        END IF;
    END PROCESS;
    
    duv_i : duv
    PORT MAP(
        a => a,
        z => z
    );
    
END ARCHITECTURE structure;
