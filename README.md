# Graphical Model Markup Language

## Syntax

```gmml

; Line comment

[BlockName]
Symbol
"String"
123
123.456
From -> To

[Definition]
Symbol = "something"
"文字列" = 42
0 = 1
"NodeA" -> "NodeB" : EdgeDefinition
ListSyntax = (a, "b", 3, 4.0, (list, in, list))
StructSyntax = {key: "value",
                "key": value,
                3: 4}
MessageCalling = Message1(something, argument)
EdgeCan -> CallMessage : message_2("foo", "bar")
```

## Spec

- Blocks are toplevel
- Case sensitive
- UTF-8
- Newline means LF(\n) or CRLF(\r\n)
