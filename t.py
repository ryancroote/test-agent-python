from agent_py import types
from agent_py import candid

b: bytes = candid.encode(
    (
        dict(
            a=types.Int(-1),
            b=types.Int8(-1),
            c=types.Int16(-1),
            d=types.Int32(-1),
            e=types.Int64(-1),
            f=types.Nat(1),
            g=types.Nat8(1),
            h=types.Nat16(1),
            i=types.Nat32(1),
            j=types.Nat64(1),
            v=types.Variant("Yes"),
        ),
    )
)

print(b.hex())

print(candid.decode(b))

decode_options = candid.DecodeOptions(
    type_names=["Basic"], definition=candid.Definition()
)

print(candid.decode(b, decode_options))
