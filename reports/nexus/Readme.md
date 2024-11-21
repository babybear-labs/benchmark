# Nexus


##  Fibonacci
```rust
#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[nexus_rt::main]
fn main() {
    let n = 7;
    let result = fib(n);
    assert_eq!(result, 13);
}
```

### Result
- Input 1
```shell
 Loading public parameters ... 6.6s
 Finished in 6.6s
  Computing step 0 ... 2.0s
  Computing step 1 ... 2.0s
  Saving proof ... 75ms
Finished in 75ms

 Loading public parameters ... 6.5s
 Finished in 6.5s
  Verifying proof ... 2.1s
   Finished in 2.1s

Proof Size: 47.9 MB
```

- Input 10
```shell
    Loading public parameters ... 6.5s
 Finished in 6.5s
  Computing step 0 ... 2.3s
  Computing step 1 ... 3.0s
  Computing step 2 ... 2.8s
  Computing step 3 ... 2.8s
  Computing step 4 ... 2.5s
  Computing step 5 ... 2.3s
  Saving proof ... 111ms
Finished in 111ms

 Loading public parameters ... 6.7s
 Finished in 6.7s
  Verifying proof ... 2.4s
   Finished in 2.4s

Proof Size: 47.9 MB
```

- Input 100
```shell
  Loading public parameters ... 6.5s
 Finished in 6.5s
  Computing step 0 ... 2.1s
  Computing step 1 ... 2.2s
  Computing step 2 ... 2.2s
  Computing step 3 ... 2.3s
  Computing step 4 ... 2.4s
  Computing step 5 ... 2.8s
  Computing step 6 ... 2.7s
  Computing step 7 ... 2.5s
  Computing step 8 ... 2.3s
  Computing step 9 ... 2.4s
  Computing step 10 ... 2.8s
  Computing step 11 ... 2.9s
  Computing step 12 ... 3.0s
  Computing step 13 ... 2.6s
  Saving proof ... 131ms
Finished in 131ms

 Loading public parameters ... 6.7s
 Finished in 6.7s
  Verifying proof ... 2.4s
   Finished in 2.4s

Proof Size: 47.9 MB
```
- Input 1000
```shell
  Loading public parameters ... 6.6s
 Finished in 6.6s
  Computing step 0 ... 2.3s
  Computing step 1 ... 3.0s
  Computing step 2 ... 2.4s
  Computing step 3 ... 1.9s
  Computing step 4 ... 2.3s
  Computing step 5 ... 2.1s
  Computing step 6 ... 1.9s
  Computing step 7 ... 2.7s
  Computing step 8 ... 2.4s
  Computing step 9 ... 2.0s
  Computing step 10 ... 2.1s
  Computing step 11 ... 2.4s
  Computing step 12 ... 2.2s
  Computing step 13 ... 2.1s
  Computing step 14 ... 2.2s
  Computing step 15 ... 2.0s
  Computing step 16 ... 2.1s
  Computing step 17 ... 2.1s
  Computing step 18 ... 2.1s
  Computing step 19 ... 2.0s
  Computing step 20 ... 2.1s
  Computing step 21 ... 2.8s
  Computing step 22 ... 2.5s
  Computing step 23 ... 2.5s
  Computing step 24 ... 2.4s
  Computing step 25 ... 2.4s
  Computing step 26 ... 3.0s
  Computing step 27 ... 2.9s
  Computing step 28 ... 2.6s
  Computing step 29 ... 2.9s
  Computing step 30 ... 2.7s
  Computing step 31 ... 2.6s
  Computing step 32 ... 3.4s
  Computing step 33 ... 2.7s
  Computing step 34 ... 2.6s
  Computing step 35 ... 2.6s
  Computing step 36 ... 2.6s
  Computing step 37 ... 2.3s
  Computing step 38 ... 2.4s
  Computing step 39 ... 2.2s
  Computing step 40 ... 2.3s
  Computing step 41 ... 2.8s
  Computing step 42 ... 2.9s
  Computing step 43 ... 2.5s
  Computing step 44 ... 2.5s
  Computing step 45 ... 3.1s
  Computing step 46 ... 3.3s
  Computing step 47 ... 3.0s
  Computing step 48 ... 3.2s
  Computing step 49 ... 2.7s
  Computing step 50 ... 2.3s
  Computing step 51 ... 2.0s
  Computing step 52 ... 2.3s
  Computing step 53 ... 2.7s
  Computing step 54 ... 2.2s
  Computing step 55 ... 2.0s
  Computing step 56 ... 2.5s
  Computing step 57 ... 2.8s
  Computing step 58 ... 2.9s
  Computing step 59 ... 4.6s
  Computing step 60 ... 2.8s
  Computing step 61 ... 2.3s
  Computing step 62 ... 2.1s
  Computing step 63 ... 2.6s
  Computing step 64 ... 2.4s
  Computing step 65 ... 2.4s
  Computing step 66 ... 2.1s
  Computing step 67 ... 1.9s
  Computing step 68 ... 2.4s
  Computing step 69 ... 2.3s
  Computing step 70 ... 2.4s
  Computing step 71 ... 2.5s
  Computing step 72 ... 2.0s
  Computing step 73 ... 2.0s
  Computing step 74 ... 3.0s
  Computing step 75 ... 2.3s
  Computing step 76 ... 2.0s
  Computing step 77 ... 2.2s
  Computing step 78 ... 2.0s
  Computing step 79 ... 2.1s
  Computing step 80 ... 1.9s
  Computing step 81 ... 2.1s
  Computing step 82 ... 2.0s
  Computing step 83 ... 2.1s
  Computing step 84 ... 2.5s
  Computing step 85 ... 2.6s
  Computing step 86 ... 3.3s
  Computing step 87 ... 5.8s
  Computing step 88 ... 8.2s
  Computing step 89 ... 11.6s
  Computing step 90 ... 6.8s
  Computing step 91 ... 6.7s
  Computing step 92 ... 7.0s
  Computing step 93 ... 7.9s
  Computing step 94 ... 28.5s
  Computing step 95 ... 41.2s
  Computing step 96 ... 1min4s
  Computing step 97 ... 22.5s
  Computing step 98 ... 3.7s
  Computing step 99 ... 3.2s
  Computing step 100 ... 2.6s
  Computing step 101 ... 3.1s
  Computing step 102 ... 4.7s
  Computing step 103 ... 6.1s
  Computing step 104 ... 10.2s
  Computing step 105 ... 10.5s
  Computing step 106 ... 20.1s
  Computing step 107 ... 33.9s
  Computing step 108 .   20.0s
    Proving [=>                       ] 108/2047: ...


```

## SHA256 (Keccak)

- Input 32 Bytes

```shell
  Loading public parameters ... 6.7s
 Finished in 6.7s
  Computing step 0 ... 2.1s
  Computing step 1 ... 2.7s
  Computing step 2 ... 2.9s
  Computing step 3 ... 2.6s
  Computing step 4 ... 3.0s
  Computing step 5 ... 3.1s
  Computing step 6 ... 2.5s
  Computing step 7 ... 2.3s
  Computing step 8 ... 2.6s
  Computing step 9 ... 2.6s
  Computing step 10 ... 3.2s
  Computing step 11 ... 2.6s
  Computing step 12 ... 2.9s
  Computing step 13 ... 3.0s
  Computing step 14 ... 2.9s
  Computing step 15 ... 2.9s
  Computing step 16 ... 2.8s
  Computing step 17 ... 2.3s
  Computing step 18 ... 3.5s
  Computing step 19 ... 3.3s
  Computing step 20 ... 3.9s
  Computing step 21 ... 2.5s
  Computing step 22 ... 2.8s
  Computing step 23 ... 3.8s
  Computing step 24 ... 2.7s
  Computing step 25 ... 2.9s
  Computing step 26 ... 3.4s
  Computing step 27 ... 2.8s
  Computing step 28 ... 2.6s
  Computing step 29 ... 2.5s
  Computing step 30 ... 2.5s
  Computing step 31 ... 2.7s
  Computing step 32 ... 3.1s
  Computing step 33 ... 3.1s
  Computing step 34 ... 3.0s
  Computing step 35 ... 2.8s
  Computing step 36 ... 2.8s
  Computing step 37 ... 2.5s
  Computing step 38 ... 2.5s
  Computing step 39 ... 2.8s
  Computing step 40 ... 2.9s
  Computing step 41 ... 3.9s
  Computing step 42 ... 3.2s
  Computing step 43 ... 3.5s
  Computing step 44 ... 3.4s
  Computing step 45 ... 2.9s
  Computing step 46 ... 2.9s
  Computing step 47 ... 2.5s
  Computing step 48 ... 4.0s
  Computing step 49 ... 3.7s
  Computing step 50 ... 4.5s
  Computing step 51 ... 4.1s
  Computing step 52 ... 3.8s
  Computing step 53 ... 3.0s
  Computing step 54 ... 3.2s
  Computing step 55 ... 2.8s
  Computing step 56 ... 2.8s
  Computing step 57 ... 3.7s
  Computing step 58 ... 3.4s
  Computing step 59 ... 2.9s
  Computing step 60 ... 2.9s
  Computing step 61 ... 2.9s
  Computing step 62 ... 2.9s
  Computing step 63 ... 2.8s
  Computing step 64 ... 2.4s
  Computing step 65 ... 2.7s
  Computing step 66 ... 2.8s
  Computing step 67 ... 3.2s
  Computing step 68 ... 2.7s
  Computing step 69 ... 2.6s
  Computing step 70 ... 2.3s
  Computing step 71 ... 2.4s
  Computing step 72 ... 2.4s
  Computing step 73 ... 3.1s
  Computing step 74 ... 2.8s
  Computing step 75 ... 2.8s
  Computing step 76 ... 3.1s
  Computing step 77 ... 2.9s
  Computing step 78 ... 2.9s
  Computing step 79 ... 2.9s
  Computing step 80 ... 2.8s
  Computing step 81 ... 2.8s
  Computing step 82 ... 4.6s
  Computing step 83 ... 2.8s
  Computing step 84 ... 2.7s
  Computing step 85 ... 4.7s
  Computing step 86 ... 4.0s
  Computing step 87 ... 3.6s
  Computing step 88 ... 3.3s
  Computing step 89 ... 2.8s
  Computing step 90 ... 2.6s
  Computing step 91 ... 3.0s
  Computing step 92 ... 2.8s
  Computing step 93 ... 2.6s
  Computing step 94 ... 2.6s
  Computing step 95 ... 2.6s
  Computing step 96 ... 3.0s
  Computing step 97 ... 2.7s
  Computing step 98 ... 2.7s
  Computing step 99 ... 3.7s
  Computing step 100 ... 2.9s
  Computing step 101 ... 4.0s
  Computing step 102 ... 2.6s
  Computing step 103 ... 2.4s
  Computing step 104 ... 2.5s
  Computing step 105 ... 2.3s
  Computing step 106 ... 2.4s
  Computing step 107 ... 2.3s
  Computing step 108 ... 2.5s
  Computing step 109 ... 2.3s
  Computing step 110 ... 2.6s
  Computing step 111 ... 2.2s
  Computing step 112 ... 2.3s
  Computing step 113 ... 2.4s
  Computing step 114 ... 3.9s
  Computing step 115 ... 3.5s
  Computing step 116 ... 3.5s
  Computing step 117 ... 3.1s
  Computing step 118 ... 2.5s
  Computing step 119 ... 3.4s
  Computing step 120 ... 3.3s
  Computing step 121 ... 2.7s
  Computing step 122 ... 2.6s
  Computing step 123 ... 2.7s
  Computing step 124 ... 4.3s
  Computing step 125 ... 2.6s
  Computing step 126 ... 2.4s
  Computing step 127 ... 2.8s
  Computing step 128 ... 3.0s
  Computing step 129 ... 2.6s
  Computing step 130 ... 2.7s
  Computing step 131 ... 2.5s
  Computing step 132 ... 2.3s
  Computing step 133 ... 2.6s
  Computing step 134 ... 3.5s
  Computing step 135 ... 3.0s
  Computing step 136 ... 2.6s
  Computing step 137 ... 2.4s
  Computing step 138 ... 3.3s
  Computing step 139 ... 2.8s
  Computing step 140 ... 2.5s
  Computing step 141 ... 2.5s
  Computing step 142 ... 2.8s
  Computing step 143 ... 3.0s
  Computing step 144 ... 3.0s
  Computing step 145 ... 3.0s
  Computing step 146 ... 3.7s
  Computing step 147 ... 2.9s
  Computing step 148 ... 3.0s
  Computing step 149 ... 3.0s
  Computing step 150 ... 3.0s
  Computing step 151 ... 3.0s
  Computing step 152 ... 3.0s
  Computing step 153 ... 4.4s
  Computing step 154 ... 3.5s
  Computing step 155 ... 3.0s
  Computing step 156 ... 3.4s
  Computing step 157 ... 2.8s
  Computing step 158 ... 2.4s
  Computing step 159 ... 2.5s
  Computing step 160 ... 2.4s
  Computing step 161 ... 2.4s
  Computing step 162 ... 2.4s
  Computing step 163 ... 2.6s
  Computing step 164 ... 2.5s
  Computing step 165 ... 2.2s
  Computing step 166 ... 2.3s
  Computing step 167 ... 2.4s
  Computing step 168 ... 2.5s
  Computing step 169 ... 2.5s
  Computing step 170 ... 2.7s
  Computing step 171 ... 2.6s
  Computing step 172 ... 2.5s
  Computing step 173 ... 2.8s
  Computing step 174 ... 2.9s
  Computing step 175 ... 2.5s
  Computing step 176 ... 2.4s
  Computing step 177 ... 2.5s
  Computing step 178 ... 3.3s
  Computing step 179 ... 3.0s
  Computing step 180 ... 4.3s
  Computing step 181 ... 4.1s
  Computing step 182 ... 2.5s
  Computing step 183 ... 2.4s
  Computing step 184 ... 2.9s
  Computing step 185 ... 3.5s
  Computing step 186 ... 2.5s
  Computing step 187 ... 2.3s
  Computing step 188 ... 2.6s
  Computing step 189 ... 2.5s
  Computing step 190 ... 2.6s
  Computing step 191 ... 3.0s
  Computing step 192 ... 3.0s
  Computing step 193 ... 2.6s
  Computing step 194 ... 2.5s
  Computing step 195 ... 2.4s
  Computing step 196 ... 4.4s
  Computing step 197 ... 5.8s
  Computing step 198 ... 4.2s
  Computing step 199 ... 3.5s
  Computing step 200 ... 3.3s
  Computing step 201 ... 2.5s
  Computing step 202 ... 2.3s
  Computing step 203 ... 2.3s
  Computing step 204 ... 2.4s
  Computing step 205 ... 2.5s
  Computing step 206 ... 2.5s
  Computing step 207 ... 2.5s
  Computing step 208 ... 2.5s
  Computing step 209 ... 2.3s
  Computing step 210 ... 2.6s
  Computing step 211 ... 2.5s
  Computing step 212 ... 2.8s
  Computing step 213 ... 3.4s
  Computing step 214 ... 2.8s
  Computing step 215 ... 2.5s
  Computing step 216 ... 2.4s
  Computing step 217 ... 2.5s
  Computing step 218 ... 2.5s
  Computing step 219 ... 2.5s
  Computing step 220 ... 2.6s
  Computing step 221 ... 2.8s
  Computing step 222 ... 2.8s
  Computing step 223 ... 3.1s
  Computing step 224 ... 3.1s
  Computing step 225 ... 2.7s
  Computing step 226 ... 2.3s
  Computing step 227 ... 2.6s
  Computing step 228 ... 2.7s
  Computing step 229 ... 2.7s
  Computing step 230 ... 2.4s
  Computing step 231 ... 2.5s
  Computing step 232 ... 2.6s
  Computing step 233 ... 2.6s
  Computing step 234 ... 2.8s
  Computing step 235 ... 3.1s
  Computing step 236 ... 2.7s
  Computing step 237 ... 2.4s
  Computing step 238 ... 2.5s
  Computing step 239 ... 2.4s
  Computing step 240 ... 2.7s
  Computing step 241 ... 3.3s
  Computing step 242 ... 2.5s
  Computing step 243 ... 2.4s
  Computing step 244 ... 2.5s
  Computing step 245 ... 2.6s
  Computing step 246 ... 2.7s
  Computing step 247 ... 3.0s
  Computing step 248 ... 2.9s
  Computing step 249 ... 2.9s
  Computing step 250 ... 2.5s
  Computing step 251 ... 2.3s
  Computing step 252 ... 2.9s
  Computing step 253 ... 3.1s
  Computing step 254 ... 4.2s
  Computing step 255 ... 2.7s
  Computing step 256 ... 2.7s
  Computing step 257 ... 2.4s
  Computing step 258 ... 2.6s
  Computing step 259 ... 3.9s
  Computing step 260 ... 4.2s
  Computing step 261 ... 4.9s
  Computing step 262 ... 3.6s
  Computing step 263 ... 3.9s
  Computing step 264 ... 4.8s
  Computing step 265 ... 2.9s
  Computing step 266 ... 4.1s
  Computing step 267 ... 4.0s
  Computing step 268 ... 4.9s
  Computing step 269 ... 4.4s
  Computing step 270 ... 3.0s
  Computing step 271 ... 2.8s
  Computing step 272 ... 2.9s
  Computing step 273 ... 2.9s
  Computing step 274 ... 2.8s
  Computing step 275 ... 2.8s
  Computing step 276 ... 2.9s
  Computing step 277 ... 3.1s
  Computing step 278 ... 2.6s
  Computing step 279 ... 2.4s
  Computing step 280 ... 2.9s
  Computing step 281 ... 3.0s
  Computing step 282 ... 2.6s
  Computing step 283 ... 2.3s
  Computing step 284 ... 3.1s
  Computing step 285 ... 3.2s
  Computing step 286 ... 5.1s
  Computing step 287 ... 3.1s
  Computing step 288 ... 2.8s
  Computing step 289 ... 2.8s
  Computing step 290 ... 2.9s
  Computing step 291 ... 3.0s
  Computing step 292 ... 2.7s
  Computing step 293 ... 3.0s
  Computing step 294 ... 2.6s
  Computing step 295 ... 2.8s
  Computing step 296 ... 4.7s
  Computing step 297 ... 3.2s
  Computing step 298 ... 3.9s
  Computing step 299 ... 2.9s
  Computing step 300 ... 2.7s
  Computing step 301 ... 3.1s
  Computing step 302 ... 5.0s
  Computing step 303 ... 2.8s
  Computing step 304 ... 3.0s
  Computing step 305 ... 3.0s
  Computing step 306 ... 2.9s
  Computing step 307 ... 4.6s
  Computing step 308 ... 2.9s
  Computing step 309 ... 3.7s
  Computing step 310 ... 3.1s
  Computing step 311 ... 5.5s
  Computing step 312 ... 4.7s
  Computing step 313 ... 3.5s
  Computing step 314 ... 3.0s
  Computing step 315 ... 3.8s
  Computing step 316 ... 3.2s
  Computing step 317 ... 2.8s
  Computing step 318 ... 3.3s
  Computing step 319 ... 2.9s
  Computing step 320 ... 4.2s
  Computing step 321 ... 2.9s
  Computing step 322 ... 3.1s
  Computing step 323 ... 2.9s
  Computing step 324 ... 3.6s
  Computing step 325 ... 3.5s
  Computing step 326 ... 2.9s
  Computing step 327 ... 4.7s
  Computing step 328 ... 2.8s
  Computing step 329 ... 3.0s
  Computing step 330 ... 2.9s
  Computing step 331 ... 3.3s
  Computing step 332 ... 3.5s
  Computing step 333 ... 4.3s
  Computing step 334 ... 4.0s
  Computing step 335 ... 3.3s
  Computing step 336 ... 3.0s
  Computing step 337 ... 3.0s
  Computing step 338 ... 4.8s
  Computing step 339 ... 3.8s
  Computing step 340 ... 3.5s
  Computing step 341 ... 3.1s
  Computing step 346 ... 834ms
    Proving [==>                      ] 346/4095: ...

Took more than 30 Minutes
```

## Merkle Merge
