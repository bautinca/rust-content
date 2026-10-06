Hay una estructura de datos muy util que aun no vimos que son las **colecciones**. La mayoria de los tipos de datos que ya vimos representan un valor especifico, pero las colecciones pueden representar un conjunto de valores. A diferencia de un array y tupla los datos a los que apuntan estas colecciones se almacenan en el **heap** lo que significa entonces que la cantidad de datos que tendra la coleccion no esta limitada por el tamaño de la memoria del stack, por tanto la cantidad de datos de la coleccion no necesita conocerse en el momento de la compilacion y puede crecer o disminuir a medida que se ejecuta el programa. Cada tipo de coleccion tiene distintas capacidades y costos y elegir el apropiado para la situacion actual es una habilidad que desarrollaremos. Veremos 3 tipos de colecciones que se usan casi siempre que son:

- **Vectores**: Nos permite almacenar un numero variable de valores uno al lado del otro

- **String**: Es una coleccion de caracteres, de hecho ya lo vimos! Es el tipo **String** que estudiamos antes, bueno este tipo de dato esta implementado como una coleccion de caracteres, por lo que podemos decir que es un vector de caracteres.

- **Hash map**: Nos permite asociar un valor con una clave especifica.

## Vectores
