from matplotlib import pyplot as plt

# function for line generation
def determineline(x0, y0, x1, y1, s):
    print(f'slope = {(y1-y0)/(x1-x0)}')
    if (y1-y0)/(x1-x0) > 1:
        return drawline(y0, x0, y1, x1, True, False, s)
    elif 0 < (y1-y0)/(x1-x0) < 1:
        return drawline(x0, y0, x1, y1, False, False, s)
    elif -1 < (y1-y0)/(x1-x0) < 0:
        print((y1-y0)/(x1-x0))
        return drawline(y0, x0, y1, x1, False, True, s)
    else:
        return drawline(-y0, x0, -y1, x1, True, True, s)

# d = sloper > 1
def drawline(x0, y0, x1, y1, d, neg, s):
    verts = []
    dx=abs(x1-x0)
    dy=abs(y1-y0)
    x=min(x0, x1)
    y=min(y0, y1)
    p=2*dy-dx

    max_x = max(x0, x1)
    print(f' start: {x}, {max_x}')

    while x <= max_x:
        if d:
            nx = y
            ny = x
        else:
            nx = x
            ny = y
        if neg:
            ny = -ny
        verts.append([nx, ny, s])

        if p >= 0:
            y=y+1
            p=p+2*dy-2*dx
        else:
            p=p+2*dy
        x=x+1

    return verts


tri = [
    [round(44.27682931495777), round(18.286878583975383)],
    [round(39.206095894036), round(11.68526512042876)],
    [round(37.099700537027594), round(16.783367572196525)]
]

points = []

print(tri)
points  = determineline(tri[0][0], tri[0][1], tri[1][0], tri[1][1], 'o')
points += determineline(tri[1][0], tri[1][1], tri[2][0], tri[2][1], 's')
points += determineline(tri[2][0], tri[2][1], tri[0][0], tri[0][1], '+')

print(points)

min_y = round(min(points, key=lambda tup: tup[1])[1])
max_y = round(max(points, key=lambda tup: tup[1])[1])

print(len(points), min_y, max_y)


for point in points:
    plt.plot(point[0], point[1], marker=point[2], markersize=5, markeredgecolor="red", markerfacecolor="green")

pairs = []

for y in range(min_y-1, max_y+1):
    pair = [y, 10000, 0]

    for point in points:
        if round(point[1]) == y:
            if point[0] < pair[1]:
                pair[1] = point[0]
            if point[0] > pair[2]:
                pair[2] = point[0]
    pairs.append(pair)

for pair in pairs:
    if pair[1] == 10000 or pair[2] == 0:
        continue
    plt.plot(pair[1], pair[0], marker="o", markersize=5, markeredgecolor="blue", markerfacecolor="green")
    plt.plot(pair[2], pair[0], marker="o", markersize=5, markeredgecolor="blue", markerfacecolor="green")

tri = [
    [44.27682931495777, 18.286878583975383],
    [39.206095894036, 11.68526512042876],
    [37.099700537027594, 16.783367572196525]
]

plt.plot((tri[0][0], tri[1][0]), (tri[0][1], tri[1][1]))
plt.plot((tri[1][0], tri[2][0]), (tri[1][1], tri[2][1]))
plt.plot((tri[2][0], tri[0][0]), (tri[2][1], tri[0][1]))

plt.grid()
plt.show()