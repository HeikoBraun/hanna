LIBRARY lib_2;
CONFIGURATION cfg_design_3 OF design_1 IS
    FOR struct
        FOR inst_i1 : des
            USE ENTITY lib_2.design_1(rtl);
        END FOR;
        FOR inst_i2 : des
            USE CONFIGURATION lib_2.cfg_design_2;
        END FOR;
    END FOR;
END CONFIGURATION cfg_design_3;
