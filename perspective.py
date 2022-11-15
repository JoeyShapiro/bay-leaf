import math
import numpy as np

a_vec = (1, 2, 0)
c_vec = (0, 0, 0)
theta = (0, 0, 0)
e_vec = (0, 0, 0)

x = a_vec[0] - c_vec[0]
y = a_vec[1] - c_vec[1]
z = a_vec[2] - c_vec[2]

cx = math.cos(theta[0])
cy = math.cos(theta[1])
cz = math.cos(theta[2])

sx = math.sin(theta[0])
sy = math.sin(theta[1])
sz = math.sin(theta[2])

dx = cy * (sz * y + cz * x) - sy * z
dy = sx * (cy * z + sy * (sz * y + cz * x)) + cx * (cz * y - sz * x)
dz = cx * (cy * z + sy * (sz * y + cz * x)) - sx * (cz * y - sz * x)

print("d: ", dx, dy, dz)

# bx = e_vec[2] / dz * dx + e_vec[0]
# by = e_vec[2] / dz * dy + e_vec[1]

# print(bx, by)

print("weak")

# y
# |  z
# | /
# |/
# +--------- x

# y
# |
# +-x
objects = []
view = []
#               x, y, z
# objects.append((4, 4, 2))
# objects.append((4, 2, 2))
# objects.append((2, 2, 2))
# objects.append((2, 4, 2))

# objects.append((4, 4, 4))
# objects.append((4, 2, 4))
# objects.append((2, 2, 4))
# objects.append((2, 4, 4))


objects.append((5, 5, 1))
objects.append((5, -4, 1))
objects.append((-4, -4, 1))
objects.append((-4, 5, 1))

objects.append((5, 5, 5))
objects.append((5, -4, 5))
objects.append((-4, -4, 5))
objects.append((-4, 5, 5))

camera = (0, 0, 0)
Bz = 15

for object in objects:
    d = math.sqrt( math.pow(object[0] - camera[0], 2) + math.pow(object[1] - camera[1], 2) + math.pow(object[2] - camera[2], 2) )

    Bx = object[0] * Bz / d
    By = object[1] * Bz / d

    print(round(Bx), round(By))
    view.append((round(Bx), round(By)))

print('-'*Bz)
for i in range(0, Bz):
    for j in range(0, Bz):
        if (j, i) in view:
            print('x', end='')
        else:
            print(' ', end='')
    print()
print('-'*Bz)

print('perspective')

height = 24
width = 80
fov = 90
zfar = 1000
znear = 0.1

a = height / width
f = 1 / math.tan(fov / 2)
scale = zfar / (zfar - znear)

projection_matrix = [
    [a*f, 0, 0, 0],
    [0, f, 0, 0],
    [0, 0, scale, 1], # the -scale and 1 are flipped in this one, but it works
    [0, 0, -scale*znear, 0]
]
np_pmat = np.array(projection_matrix)

viewPort = []

for object in objects:
    p = np.array(object + (1, ))

    p[2] += 3

    # multiply by perspective matrix
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