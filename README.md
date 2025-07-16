# discord-mismatch
Decodes UTF-8 text mismatched as UTF-16, helpful you try to SMS verify but not in English.
# why?
bcuz sometimes when u make phone verification in another language ur SMS client fucks itself up and sends you shit like this
```嘀漀琀爀攀 挀漀搀攀 搀攀 猀挀甀爀椀琀 䐀椀猀挀漀爀搀 攀猀琀ꀀ㨀 㘀㜀㘀㤀㤀　```
# how to recognize the numbers
1. **Get the UTF-16 bytes of the string.**
2. **Check each 2-byte pair (one UTF-16 char):**
3. **Is the number stored in the high byte (first byte) or the low byte (second byte)?**
ASCII digits 0 to 9 are hex 0x30 to 0x39.
5. **Extract those bytes and convert to characters.**
> remember that u can js paste ur whole msg into the first variable
