from hilbertcurve.hilbertcurve import HilbertCurve
p=8; n=2
hilbert_curve = HilbertCurve(p, n)
points = hilbert_curve.distance_from_point([2**16 - 1, 0])
print(points);
