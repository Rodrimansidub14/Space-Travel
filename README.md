# Simulación del Sistema Solar en Rust

## Descripción

Este proyecto es una simulación interactiva del sistema solar desarrollada en Rust utilizando un renderer personalizado. La simulación incluye un sol y varios planetas que orbitan alrededor de él, alineados en el plano eclíptico. Cada planeta no solo se traslada en una órbita circular, sino que también rota sobre su propio eje, ofreciendo una representación realista del movimiento planetario.

Además, se ha implementado una cámara que permite explorar el sistema solar en tercera persona. La cámara puede moverse sobre el plano eclíptico, y es posible seleccionar diferentes planetas para seguirlos de cerca. Al seleccionar un planeta, se inicia una animación de warp que simula un efecto de "warp drive", trasladando la vista a una perspectiva en tercera persona que sigue al planeta seleccionado. Al finalizar el warp, la vista se redirige suavemente al planeta, deteniendo su órbita durante la selección para una observación detallada.

## Características

- **Renderizado de Cuerpos Celestes:**
  - Un sol y múltiples planetas alineados en el plano eclíptico.
  - Rotación de planetas sobre su propio eje.
  - Varias clases de planetas, incluyendo planetas rocosos, gigantes gaseosos, con anillos, lunas y cometas.

- **Cámara Interactiva:**
  - Movimiento libre sobre el plano eclíptico.
  - Seguimiento dinámico de planetas seleccionados.
  - Animación de warp para transiciones suaves entre vistas.

- **Efectos Visuales:**
  - Skybox que simula un campo estelar en el horizonte.
  - Evitación de colisiones entre la nave/cámara y los cuerpos celestes.
  - Renderizado de órbitas planetarias como líneas de puntos.

- **Interfaz de Usuario:**
  - Selección de planetas mediante teclas numéricas.
  - Transiciones animadas al cambiar de planeta.
  - Pausa de la órbita durante la selección de un planeta para una observación detallada.

## Requisitos

- **Lenguaje de Programación:** Rust
- **Dependencias:**
  - `nalgebra_glm`
  - `minifb`
  - `tobj`
  - `fastnoise_lite`
  - Otras dependencias necesarias según el proyecto.

## Instalación y Ejecución

1. **Clonar el Repositorio:**
   
   ```bash
   git clone https://github.com/tu-usuario/sistema-solar-rust.git
   cd sistema-solar-rust
   
## Construir el Proyecto

Asegúrate de tener Rust instalado. Si no lo tienes, puedes instalarlo desde [rustup.rs](https://rustup.rs/).

```bash
cargo build --release
```
## Ejecutar la Simulación
```bash

cargo run --release

```
## Controles
### Movimiento de la Cámara:
Rotar: Haz clic y arrastra con el botón izquierdo del ratón para orbitar la cámara alrededor del sistema solar.
Zoom: Utiliza la rueda del ratón para acercar o alejar la vista.
## Selección de Planetas:
Teclas Numéricas (1-9): Presiona una tecla numérica para seleccionar y seguir el planeta correspondiente.
Otros Controles:
Cerrar Aplicación: Presiona la tecla Escape para cerrar la simulación.

### Video de Demostración

https://youtu.be/R-ur2ixCy_g
