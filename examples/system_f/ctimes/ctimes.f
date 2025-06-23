def main := \m:(forall X.(X->X)->X->X).
  \n:(forall X.(X->X)->X->X).
    \X.\s:X->X.(n[X])((m[X])(s));
