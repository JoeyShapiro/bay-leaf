points = [
    (0, 0, 0, 1),
    (1, 0, 0, 1),
    (0, 1, 0, 1),
    (1, 1, 0, 1),
    (0, 0, 1, 1),
    (1, 0, 1, 1),
    (0, 1, 1, 1),
    (1, 1, 1, 1),
]

import pywavefront
scene = pywavefront.Wavefront('bunny.obj', strict=True, encoding="utf-8", collect_faces=True)
# scene.parse()  # Explicit call to parse() needed when parse=False

print(scene.mesh_list[0].faces)
print(len(scene.vertices))

triangles = []

# for face in scene.mesh_list[0].faces:
#     triangle = []
#     for point in face:
#         triangle.append(scene.vertices[point])
#     triangles.append(triangle)

# can just reuse what they have

print(triangles)

lines = []

# Create points
#let a = point::Point { x: 0.0, y: 1.0, z: 0.0, w: 1.0 };
for i, point in enumerate(scene.vertices):
    # print(f'let p{i} = point::Point {{ x: {point[0]}, y: {point[1]}, z: {point[2]}, w: 1.0 }};')
    lines.append(f'let p{i} = point::Point {{ x: {point[0]}, y: {point[1]}, z: {point[2]}, w: 1.0 }};\n')

# Create triangles
# triangles.push(triangle::Triangle { a: a, b: b, c: c });
print('\n')
lines.append('\n')

for face in scene.mesh_list[0].faces:
    # print(f'triangles.push(triangle::Triangle {{ a: p{face[0]}, b: p{face[1]}, c: p{face[2]} }});')
    lines.append(f'triangles.push(triangle::Triangle {{ a: p{face[0]}, b: p{face[1]}, c: p{face[2]} }});\n')

with open('out.dat', 'w') as f:
    f.writelines(lines)
    
# let a = point::Point { x: 0.0, y: 1.0, z: 0.0, w: 1.0 };
# let b = point::Point { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
# let c = point::Point { x: 1.0, y: 0.0, z: 0.0, w: 1.0 };
# let d = point::Point { x: 1.0, y: 1.0, z: 0.0, w: 1.0 };
# let e = point::Point { x: 0.0, y: 0.0, z: 1.0, w: 1.0 };
# let f = point::Point { x: 0.0, y: 1.0, z: 1.0, w: 1.0 };
# let g = point::Point { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
# let h = point::Point { x: 1.0, y: 0.0, z: 1.0, w: 1.0 };

# // south
# triangles.push(triangle::Triangle { a: a, b: b, c: c });
# triangles.push(triangle::Triangle { a: a, b: d, c: c });
# // east
# triangles.push(triangle::Triangle { a: f, b: e, c: b });
# triangles.push(triangle::Triangle { a: f, b: a, c: b });
# // north
# triangles.push(triangle::Triangle { a: g, b: h, c: f });
# triangles.push(triangle::Triangle { a: g, b: e, c: f });
# // west
# triangles.push(triangle::Triangle { a: c, b: d, c: h });
# triangles.push(triangle::Triangle { a: c, b: g, c: h });
# // top
# triangles.push(triangle::Triangle { a: f, b: a, c: c });
# triangles.push(triangle::Triangle { a: f, b: g, c: c });
# // bottom
# triangles.push(triangle::Triangle { a: h, b: d, c: b });
# triangles.push(triangle::Triangle { a: h, b: e, c: b });