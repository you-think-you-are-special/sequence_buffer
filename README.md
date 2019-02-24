# Sequence buffer

Sequence buffer data structure implementation


# Properties:

- Constant time insertion for a given number (inserts may be random)
- Constant time query for a given number
- Constant time access for the data stored for a given number
- Constant time removal of entries


# Examples

```rust
  let mut buf = SequenceBuffer::new(1);
  
  buf.insert(DataStub, 543535);
  assert!(buf.exists(543535));
 
  buf.insert(DataStub, 2535436);
  buf.remove(2535436);
  assert!(!buf.exists(2535436));
```

# Use cases

- Ack system in netwrok protocol
