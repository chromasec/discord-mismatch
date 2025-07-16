niggers = "㜀㌀㜀㈀㠀㠀??"  # replace by ur 8 bit chars.
slaves = niggers.encode('utf-16-be')
print("UTF-16 BE bytes (hex):", slaves.hex())
acceptance = slaves.decode('ascii', errors='ignore')
print("Original bytes (ASCII):", acceptance)
# please note that this isnt accurate.
