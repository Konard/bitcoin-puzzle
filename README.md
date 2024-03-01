# bitcoin-puzzle
A bitcoin puzzle solution

## Example

```bash
hashcat -a 3 -m 30901 13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so "000000000000000000000000000000000000000000000003000000?h?h?h?h?h?h?h?h?h?h" --markov-disable -d 2
```

## Progress

### 13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so

| Range                                                                        | Result |
| ---------------------------------------------------------------------------- | ------ |
| `000000000000000000000000000000000000000000000002ffffff?h?h?h?h?h?h?h?h?h?h` | `○`    |
| `000000000000000000000000000000000000000000000003000000?h?h?h?h?h?h?h?h?h?h` | `?`    |
