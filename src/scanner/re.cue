package scanner

#DigitSeparator: "'"
#MagnitudeOrder: "(K|M|G|T|Ki|Mi|Gi|Ti)"

#IntegerPattern: {
    prefix: string | *""
    pattern: string
    out: "\(prefix)\(pattern)+(\(#DigitSeparator)\(pattern)+)*"
}

re: {
    DecimalInteger:     (#IntegerPattern & {               pattern: "[0-9]"}).out
    HexadecimalInteger: (#IntegerPattern & { prefix: "0x", pattern: "[0-9a-fA-F]"}).out
    OctalInteger:       (#IntegerPattern & { prefix: "0o", pattern: "[0-7]"}).out
    BinaryInteger:      (#IntegerPattern & { prefix: "0b", pattern: "(0|1)"}).out
    Integer:            "(\(DecimalInteger))|(\(HexadecimalInteger))|(\(OctalInteger))|(\(BinaryInteger)))"
    BitSize:            "(\(Integer)b)"
}