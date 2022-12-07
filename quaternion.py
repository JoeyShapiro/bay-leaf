import math
import numpy as np

points = []
points.append((1, 1, -1))
points.append((1, -1, -1))
points.append((1, 1, 1))
points.append((1, -1, 1))

points.append((-1, 1, -1))
points.append((-1, -1, -1))
points.append((-1, 1, 1))
points.append((-1, -1, 1))

# q  = 0.866 + 1i + 0j + -1k
# q- = 0.866 - 1i - 0j - -1k
# qv = 1i + 1j + 0k
# p' = (0.866 + 1i + 0j + -1k) * (1i + 1j + 0k) * (0.866 - 1i - 0j - -1k)
#    = (-1 + 1.866i - 0.134j + k) * (0.866 - 1i - 0j - -1k)

def Q(theta, x, y, z):
    q = np.zeros((3, 3))
    theta_rad = math.radians(theta / 2)
    l = math.sqrt(x*x + y*y + z*z)

    a = math.cos(theta_rad)
    b = x/l * math.sin(theta_rad)
    c = y/l * math.sin(theta_rad)
    d = z/l * math.sin(theta_rad)

    q[0][0] = round(1 - 2 * (c*c + d*d), 3)
    print(f'1 - 2 * ({c}*{c} + {d}*{d})')
    q[0][1] = round(2 * (b * c - a * d), 3)
    q[0][2] = round(2 * (b * d + a * c), 3)
    # q[0][3] = round(0

    q[1][0] = round(2 * (b*c + a*d), 3)
    q[1][1] = round(1 - 2 * (b*b + d*d), 3)
    q[1][2] = round(2 * (c*d - a*b), 3)
    # q[1][3] = round(0

    q[2][0] = round(2*(b*d - a*c), 3)
    q[2][1] = round(2*(c*d + a*b), 3)
    q[2][2] = round(1 - 2*(b*b + c*c), 3)
    # q[2][3] = round(0

    # q[3][0] = round(0
    # q[3][1] = 0
    # q[3][2] = 0
    # q[3][3] = 1

    return q

q = Q(60, 1, 0, -1)
print(q)

p = np.array(( 1, 1, 0 ))#points[0])
q_i = np.linalg.inv(q)

# print(np.dot(q, p))
# print(p.reshape(3, 1))
print('test2', np.dot(q, p)) # you only need the q, no inverse for matrix
print('test', np.dot(np.dot(q, p), q_i))


# nubmerphile
theta = math.radians(0)
v = [1, 0, 0]

a = math.cos(theta / 2)
b = v[0] * math.sin(theta / 2)
c = v[1] * math.sin(theta / 2)
d = v[2] * math.sin(theta / 2)

print(math.sin(theta / 2))

h = np.array((a, b, c, d))
h_star = np.array((a, -b, -c, -d))

height = 24
width = 80
fov = 90
zfar = 1000
znear = 0.1


a = height / width
f = 1 / math.tan(math.radians(fov / 2))
scale = zfar / (zfar - znear)

projection_matrix = [
    [a*f, 0, 0, 0],
    [0, f, 0, 0],
    [0, 0, scale, 1], # the -scale and 1 are flipped in this one, but it works
    [0, 0, -scale*znear, 0]
]
np_pmat = np.array(projection_matrix)

viewPort = []
q = Q(60, 1, 0, 0)

for point in points:
    tp = np.array(point)
    tp[2] += 3

    tp2 = np.dot(q, tp)
    
    p = np.array((tp2[0], tp2[1], tp2[2], 1))

    # p[2] += 3
    pp = np.matmul(p, np_pmat)
    if pp[3] == 0:
        continue
    
    # convert 4d -> 3d
    pp[0] /= pp[3]
    pp[1] /= pp[3]
    pp[2] /= pp[3]
    pointNew = pp[:3]

    # scale into view
    pointNew[0] += 1
    pointNew[1] += 1

    pointNew[0] *= 0.5 * width
    pointNew[1] *= 0.5 * height

    print(pointNew)
    # convert the x and y to a rounded tuple
    viewPort.append(tuple(np.round(pointNew, 0)[:2]))

print('-'*width)
for i in range(height):
    for j in range(width):
        if (j, i) in viewPort:
            print('x', end='')
        else:
            print(' ', end='')
    print()
print('-'*width)

# quaternion
theta_rad = math.radians(60 / 2)
l = math.sqrt(1 + 0 + 1)

a = math.cos(theta_rad)
b = 1/l * math.sin(theta_rad)
c = 0/l * math.sin(theta_rad)
d = -1/l * math.sin(theta_rad)
print(f'q(60, (1, 0, -1)) = {round(a, 3)} + {round(b, 3)}i + {round(c, 3)}j + {round(d, 3)}k')

# l = math.sqrt(1 + 1 + 0)

# i was converting this to quaternion, should be nothing
e = 0# math.cos(0)
f = 1#/l * math.sin(0)
g = 1#/l * math.sin(0)
h = 0#/l * math.sin(0)
print(f'q(0, (1, 1, 0)) = {round(e, 3)} + {round(f, 3)}i + {round(g, 3)}j + {round(h, 3)}k')

a1 = a*e-b*f-c*g-d*h
b1 = a*f+b*e+c*h-d*g
c1 = a*g-b*h+c*e+d*f
d1 = a*h+b*g-c*f+d*e
print(f'q * p = {round(a1, 3)} + {round(b1, 3)}i + {round(c1, 3)}j + {round(d1, 3)}k')

e1 = a
f1 = -b
g1 = -c
h1 = -d

a2 = a1*e1-b1*f1-c1*g1-d1*h1
b2 = a1*f1+b1*e1+c1*h1-d1*g1
c2 = a1*g1-b1*h1+c1*e1+d1*f1
d2 = a1*h1+b1*g1-c1*f1+d1*e1
print(f'(q * p) * q\' = {round(a2, 3)} + {round(b2, 3)}i + {round(c2, 3)}j + {round(d2, 3)}k')
