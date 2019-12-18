# Region QuadTree para compresión
El repositorio incluye la estructura de datos de Region QuadTree 
implementada en Rust para comprimir imágenes.

# Pruebas
Se probó con dos imágenes, las que están en la carpeta `test, los 
resultados son presentados a continuación

## Prueba 1

**Imagen original:**
![Original](test/test.png)

**Imagen con rectángulos**
![Con rectángulos](test/output.png)

## Prueba 2

**Imagen original:**
![Original](test/test2.png)

**Imagen con rectángulos**
![Con rectángulos](test/output2.png)

# Reproducción
Para probar el código se necesita tener Rust 1.39, y para ejecutar el programa se debe ejecutar el siguiente comando:

```
$ cargo run -- <imagen> <salida> <presicion>
```