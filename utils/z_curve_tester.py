import pymorton as pm

morton_number = pm.interleave(5, 6)
print(morton_number)
morton_number = pm.interleave(0, 3)
print(morton_number)
morton_number = pm.interleave(21, 42)
print(morton_number)
