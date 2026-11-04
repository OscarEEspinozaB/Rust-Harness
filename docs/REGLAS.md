# Reglas de Rust-Harness

Una regla existe cuando `rust-harness` rechaza el código que la incumple. Este documento dice qué comprueba cada una y qué no. Las premisas citadas son las de Zymbol, en `zymbol-design/PREMISES.md`.

## Cómo se aplican

El programa combina dos capas, y un archivo pasa solo si pasa ambas:

- **Reglas de sintaxis** ([src/audit.rs](../src/audit.rs)): leen cada archivo con `syn`. Funcionan en la cadena estable y no conocen los tipos.
- **Clippy y el compilador**, con [clippy.toml](../clippy.toml) y los lints de `ENFORCED_LINTS`: conocen los tipos, pero dependen de listas de nombres.

```sh
rust-harness [--syntax-only] [--exclude NOMBRE]... RUTA...
```

Termina con 0 si no hay hallazgos, 1 si los hay y 2 si no pudo auditar, por ejemplo si no encontró archivos o falta `clippy-driver`. Una auditoría que no leyó nada nunca termina con 0.

En una carpeta sin `Cargo.toml` solo corren las reglas de sintaxis, y el programa lo avisa.

## Reglas

| Regla | Premisa | Rechaza | No detecta |
|---|---|---|---|
| `STATE` | MEM-1, MEM-2 | Todo `static`, incluidos los `static` inmutables, los locales y los de `extern` o del texto de una macro. Tipos con mutabilidad interior de `std` | Estado global dentro de una dependencia. Tipos de otras bibliotecas que no estén listados |
| `WORLD` | AGT-4, MEM-5 | Fuera de `fn main`: `std::env`, `fs`, `net`, `os`, `process`, `stdin`, `stdout`, `stderr`, los relojes y las macros de impresión, también con alias, `use` encadenados, globs o dentro de una macro | Dependencias que tocan el exterior. Métodos sin listar alcanzados por un alias de tipo (celda `world_type_alias_unlisted_method`) |
| `NAME` | MEM-7 | Parámetro llamado como su función. Variable y función anidada con el mismo nombre. Ocultar un nombre (Clippy) | Nombres generados por macros |
| `LAMBDA` | MEM-6 | Una closure que asigna a un nombre que no declaró, o toma `&mut` de él | Mutación mediante un método, como `out.push(x)` (celda `lambda_mutates_through_method`) |
| `CONTEXT` | MEM-5 | `&mut` que no se escribe. Más de 7 parámetros | Llamada que reenvía un `&mut` sin marca visible (celda `context_mut_reborrow_unmarked`). Estructura usada solo en parte (`context_unused_fields`) |
| `BYPASS` | — | `allow`, `expect`, `warn` o `cfg_attr` que relajen una regla o un grupo que la contenga | Lints desactivados desde fuera del código, por ejemplo con `RUSTFLAGS` |
| `HIDDEN` | — | `unsafe` | `impl Drop` y `macro_rules!`: siguen como huecos, sin decidir |
| `RELEASE` | MEM-8 | `mem::forget` | Liberar tarde (`release_late_drop`) |

## Decisiones de diseño

- **La excepción es `fn main`, y solo esa.** Debe estar en el nivel superior del archivo, no ser pública, no tener parámetros y no estar en `lib.rs`. Las closures que crea pueden bajar como parámetros: así lo hace el propio programa.
- **Un `static` inmutable también se rechaza**, porque sin tipos no se distingue de uno con mutabilidad interior y `const` lo reemplaza. La excepción es `#[global_allocator]`, que el lenguaje solo admite como `static`.
- **Los bloques hermanos pueden repetir un nombre.** Según MEM-6, lo que nace en un bloque muere con él. Lo prohibido es ocultar un nombre que ya existe.
- **Un tipo del exterior puede pasar como parámetro.** `fn f(file: &mut File)` es correcto, porque está en la firma; `File::open` fuera de `main` no lo es.
- **Una closure puede leer lo que la rodea, pero solo escribe lo que declara**, como en MEM-6. Un método de una variable capturada también escribe, pero detectarlo requiere tipos.

## Límites

- **La lista del exterior no está cerrada.** AGT-4 se apoya en una lista cerrada, que Zymbol controla porque es su lenguaje. En Rust, cualquier dependencia puede tocar el exterior sin que esto lo vea.
- **Una macro de otra biblioteca no se expande** en la capa de sintaxis. Clippy sí ve los métodos y tipos que resultan, si están en sus listas.
- **`include!` no se sigue**, y las macros como `env!` e `include_str!` leen en compilación sin rechazo.
- **Sin revisar:** `build.rs`, macros de procedimiento y bloques `#[cfg]` que no se compilan en la configuración auditada, aunque la capa de sintaxis sí los lee.

## Autoauditoría

`./run.sh` ejecuta `rust-harness --exclude cells .` sobre este repositorio, y su resultado cuenta como un veredicto más. Se excluye `cells/` porque esas celdas incumplen las reglas a propósito.

Esta autoauditoría se probó el 3 de noviembre de 2026 introduciendo defectos en una copia del código: acceso a `env` en una función auxiliar, `static`, escritura en una closure, `allow` en todo el crate, quitar el `allow` de `main`, ocultar un nombre, `println!`, `Cell` y un error de sintaxis. Los 9 hicieron fallar la auditoría por el motivo esperado.
