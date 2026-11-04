# Rust-Harness

**¿Se puede hacer que una función de Rust dependa solo de lo que dice su firma?** Este proyecto busca que, para entender una función, baste con leer su firma y su cuerpo, y comprueba esas reglas con un programa.

Rust ya rechaza los errores de memoria en código seguro, así que esto no pretende mejorar esa seguridad. Busca otra cosa: menos estado oculto y menos código oculto, para que una persona o una IA puedan construir sin leer todo el proyecto. **Que esto ayude de verdad a una IA es una hipótesis sin medir.**

Nació de [C-Harness](https://github.com/OscarEEspinozaB/C-Harness), que concluyó que estas garantías no se pueden dar en C sin convertirlo en otro lenguaje. Las reglas vienen de las premisas de diseño del lenguaje Zymbol.

## Estado actual

Es un experimento en una etapa temprana. Existe el validador `rust-harness`, que se audita a sí mismo, pero **no garantiza que un programa cumpla la propiedad completa**.

## Qué rechaza

| Regla | Qué rechaza |
|---|---|
| `STATE` | Variables globales: cualquier `static`. Solo las constantes pueden ser globales |
| `WORLD` | Acceder al exterior fuera de `fn main`: archivos, entorno, red, consola o reloj. Se obtiene en `main` y baja por parámetros |
| `NAME` | Que un nombre designe dos cosas dentro de una función, y ocultar un nombre |
| `LAMBDA` | Una closure que escribe un nombre que no declaró |
| `CONTEXT` | Un `&mut` que no se escribe, o más de 7 parámetros |
| `BYPASS` | Apagar una regla desde el propio código con `#[allow]` |

El detalle de lo que comprueba cada regla, **y de lo que no**, está en [docs/REGLAS.md](docs/REGLAS.md). Los límites más importantes:

- Lo que hace una dependencia por dentro no se ve, así que no se puede afirmar que la lista de accesos al exterior esté cerrada.
- Una closure que muta con un método, como `out.push(x)`, pasa, porque detectarlo requiere tipos.
- `impl Drop` y `macro_rules!` todavía se permiten.

## Uso

Necesitas la cadena estable de Rust con Clippy.

```sh
cargo build
target/debug/rust-harness RUTA                # audita un archivo o un proyecto
target/debug/rust-harness --syntax-only RUTA  # sin compilar, solo las reglas de sintaxis
```

Termina con 0 si no hay hallazgos, con 1 si los hay y con 2 si no pudo auditar.

## Reproducir la medición

```sh
./run.sh
```

Mide las celdas de [cells/](cells/) y la autoauditoría, y termina con error si un veredicto es incorrecto. Hay mediciones registradas en [results/](results/).

Las celdas son programas muy pequeños escritos junto con las reglas. Son pruebas de regresión, **no una tasa de detección**: no dicen cuánto detecta el programa en código real.

## Idiomas

El código está en inglés: identificadores, comentarios, mensajes y nombres de archivo. La documentación está en español.

## Licencia

Aún no se ha elegido. Hasta entonces, el proyecto no concede ninguna licencia sobre su código.
