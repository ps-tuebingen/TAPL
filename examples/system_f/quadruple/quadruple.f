def main := \X.(
 ((\X.(\f:X->X.\a:X.(f) ((f) (a))))[X -> X])
 ((\X.(\f:X->X.\a:X.(f) ((f) (a))))[X])
);
