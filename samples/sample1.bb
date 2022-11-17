A: NUMBER = 10;
B: NUMBER = 20;
C: STRING = "ABCDEFG";
D: STRING = "ABCDEFGH";
PUT (C | D);
IF A < B {
    T: STRING = "A is less than B";
    PUT T;
} ELSE {
    TT: NUMBER = A % B;
    PUT TT;
};
WHILE A > 0 {
    A = A - 1;
    IF A < 5 {
        PUT A;
    };
};
