# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Rust-Harness es un arnés para escribir Rust limitando el contexto: que para entender una función baste con leer su firma y su cuerpo. La propiedad central es:

> **Una función solo cambia lo que su firma dice que cambia.**

Rust ya rechaza los errores de memoria en código seguro, así que este arnés no se presenta como una mejora de la seguridad de memoria. Aporta otra cosa: menos estado oculto y menos código oculto, para que una IA o una persona construyan sin tener que leer todo el proyecto. **Que esto ayude de verdad a una IA es una hipótesis sin medir.**

**Estado:** existe el validador `rust-harness`, que combina reglas de sintaxis propias con Clippy y se audita a sí mismo. Lo que comprueba cada regla y lo que no está en [docs/REGLAS.md](docs/REGLAS.md). Es público en [OscarEEspinozaB/Rust-Harness](https://github.com/OscarEEspinozaB/Rust-Harness).

Las reglas vienen de las premisas de Zymbol, en `zymbol-design/PREMISES.md` del proyecto de ese lenguaje. Ante una duda sobre qué debe hacer una regla, esas premisas pesan más que una propuesta de este repositorio.

## Reglas de trabajo

- **Una regla es un programa.** El autor pidió expresamente no escribir premisas o reglas que no se puedan garantizar. Una regla existe cuando una herramienta rechaza el código que la incumple; antes de eso es, como mucho, una celda de tipo `gap`.
- **Di qué comprueba cada regla y qué no.** No afirmes garantías más amplias que lo medido.
- **Primero se pasa por sí mismo.** Todo el código de Rust-Harness debe superar sus propias reglas.
- **Cada defecto se corrige junto con una celda que lo vigile.**
- **Un rechazo solo vale si es por el motivo esperado.**
- **Los resultados de las celdas son pruebas de regresión, no una tasa de detección.** No los presentes como tal. Lo revelador es probar con programas reales.
- **Un programa estudiado es evidencia, no modelo.** Un hallazgo en él se revisa antes de atribuirlo: puede ser un defecto del arnés.
- **Los programas estudiados no están en falta.** Estas reglas nunca se les exigieron. No modifiques esos proyectos desde aquí.

## Idiomas

- **Código en inglés:** identificadores, comentarios, mensajes, tablas que generan los scripts y nombres de archivos de código.
- **Documentación en español:** README y `docs/`. Los nombres de código se citan tal cual. Evita mezclar los dos idiomas en el código.
- **Mensajes de commit:** en español, como en C-Harness.

## Comandos

```sh
./run.sh                                    # mide las celdas y la autoauditoría; termina con error si un veredicto es incorrecto
./run.sh > results/...                      # para registrar una medición, en un archivo nuevo con fecha y sufijo
target/debug/rust-harness RUTA              # audita un archivo o un proyecto
target/debug/rust-harness --syntax-only RUTA  # sin compilar, solo las reglas de sintaxis
```

Requiere `cargo` y `clippy-driver` en la cadena estable. Hoy solo está instalada la cadena estable, con Rust y Clippy 1.99; también está `cargo-miri`. Las reglas de sintaxis usan `syn`, así que no requieren nightly.

Al auditar con Clippy un proyecto ajeno, define `CARGO_TARGET_DIR` fuera de él para no ensuciarlo.

## Celdas (`cells/*.rs`)

La primera línea declara la regla y el tipo, y la segunda, el motivo esperado si debe detenerse:

```rust
// rule: STATE kind: violation
// reason: disallowed type
```

| `kind` | Veredicto correcto |
|---|---|
| `control` | Programa válido: debe pasar |
| `violation` | Se detiene y el mensaje contiene `reason`. Si se detiene por otra razón, el veredicto es `**REASON**` |
| `gap` | Debería detenerse, pero aún no hay regla: hoy pasa. Si llega a detenerse, el veredicto es `**REVIEW**`, y hay que convertirla en `violation` |

## Repositorio

- **Identidad:** el repositorio usa como `user.email` `219075297+OscarEEspinozaB@users.noreply.github.com`. Si lo clonas de nuevo, configúralo antes de hacer commits.
- **GitHub:** la cuenta es `OscarEEspinozaB`. Comprueba con `gh auth status` que sea la activa, porque hay más de una.
- **Licencia:** está sin decidir, así que no hay archivo `LICENSE`. Una licencia concedida no se retira, así que no añadas una sin que el autor la elija.
- **Lo que no se publica:** revisa `.gitignore` y `CLAUDE.local.md` antes de añadir archivos nuevos. Busca rutas locales y datos privados en lo que vayas a subir.
