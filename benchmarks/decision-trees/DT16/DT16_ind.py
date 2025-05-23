from sppl.compilers.sppl_to_python import SPPL_Compiler
from sppl.compilers.ast_to_spe import Id

compiler = SPPL_Compiler('''
# Population model
age ~= norm(loc=38.5816, scale=186.0614)

relationship ~= choice({
    '0': 0.0481,
    '1': 0.1557,
    '2': 0.4051,
    '3': 0.2551,
    '4': 0.0301,
    '5': 0.1059
})

sex ~= choice({'female': 0.3307, 'male': 0.6693})

capital_gain ~= norm(loc=1077.6488, scale=54542539.1784)

condition(sex == 'female')
condition(age > 18)

# Decision logic
if relationship == '0':
    if capital_gain < 5095.5:
        t ~= atomic(loc=1)
    else:
        t ~= atomic(loc=0)
elif relationship == '1':
    if capital_gain < 4718.5:
        t ~= atomic(loc=1)
    else:
        t ~= atomic(loc=0)
elif relationship == '2':
    if capital_gain < 5095.5:
        t ~= atomic(loc=1)
    else:
        t ~= atomic(loc=0)
elif relationship == '3':
    if capital_gain < 8296.0:
        t ~= atomic(loc=1)
    else:
        t ~= atomic(loc=0)
elif relationship == '4':
    t ~= atomic(loc=1)
else:
    if capital_gain < 4668.5:
        t ~= atomic(loc=1)
    else:
        t ~= atomic(loc=0)
''')

n = compiler.execute_module()
t = Id('t')
event = (t < 0.5)
print(n.model.prob(event))
