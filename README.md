# Proyecto de computación distribuida

## Introducción
Este es un proyecto para la materia *Programación de Sistemas Avanzados* impartida en el Centro Universitario
de Ciencias Exactas e Ingenierías (CUCEI) por el profesor Jorge Ernesto Lopez Arce Delgado. El proyecto
consiste en implementar un algoritmo paralelizable mediante computación distribuida.
Para lograr lo anterior,  se configuró una red privada virtual (VPN) por medio de
*WireGuard*, una herramienta de código abierto que implementa redes privadas virtuales
encriptadas. La topología de red implementada es de tipo “estrella”: todos los clientes de la
red se conectan a un nodo central, o hub, formando una estrella.
Cada cliente, e incluso el hub, ejecuta cuatro contenedores de *Docker*. Estos contenedores
también estarán conectados a la red. Cada contenedor pedirá al hub una tarea; el hub
indicará la tarea al contenedor; una vez que el contenedor recibe su tarea, la ejecuta y envía
el resultado obtenido al hub.
El algoritmo se implementará en el lenguaje de programación *Rust*, un lenguaje de
propósito general muy eficiente y seguro en cuanto a memoria se refiere.
Con el fin de organizar eficientemente las tareas a realizar de la actividad, con ayuda de
*ClickUp*, un project management software, se hará uso de dos metodologías ágiles: *Scrum* y
*Kanban*.

## Objetivo
El objetivo de este proyecto es implementar y lograr la paralelización de un algoritmo,
programado en lenguaje *RUST*, mediante la computación distribuida, utilizando un red
virtual privada (VPN) configurada con *WireGuard*. Además, se busca integrar contenedores
*Docker* para la ejecución distribuida de tareas.
Así mismo, se busca la implementación de metodologías ágiles, en este caso, *Scrum* y
*Kanban* para la organización y estructuración del proyecto.

## Desarrollo
### Cálculo de pi por medio del método Monte Carlo
La implementación del algoritmo del cálculo de pi por método de Monte Carlo resulta en una
prueba definitiva para maximizar el uso del CPU de cada máquina y comprobar el beneficio
que contrae el distribuir la carga de trabajo entre los nodos incluidos por consecuencia, no
solo se refleja un mejor desempeño en relación a la velocidad de procesamiento, como es el
caso del algoritmo Mandelbrot, si no que consigue un mejor resultado en función a los
nodos que se añadan (siendo la comprobación de la utilidad de los sistemas distribuidos el
propósito principal de esta actividad), logrando así una mayor precisión matemática,
valiosísima en el ámbito científico y computacional.
Debido a la naturaleza de este algoritmo no se necesita estar en comunicación constante
entre los nodos, ya que la generación aleatoria de números se hace de manera
independiente por cada nodo,llevando al máximo el CPU de cada máquina, y
posteriormente de que se hayan generado estos datos, ahora sí se procederá con la
comunicación entre nodos para compartir la cantidad de números clasificados como “dentro
del círculo” y los números totales generados, para poder realizar la aproximación final.
El cálculo de π mediante el método Monte Carlo se utiliza como un problema de referencia
para evaluar el desempeño del sistema distribuido desarrollado. Debido a que el resultado
esperado es conocido y el problema es inherentemente paralelizable, permite analizar
métricas fundamentales como escalabilidad, balance de carga y eficiencia computacional
sin introducir dependencias externas.
El objetivo no es obtener un valor más preciso de π por necesidad práctica, sino utilizar un
modelo matemático controlado que represente simulaciones probabilísticas reales
empleadas en cómputo científico distribuido.

## Planeación y coordinación
### Metodología
Para nuestro proyecto elegimos una combinación de dos metodologías: Scrum y Kanban.
Tomamos esta decisión porque fue recomendada por dos miembros del equipo, ya que la
habían utilizado con anterioridad y mencionan que la combinación de ambas metodologías
mejora la organización y la comunicación. Además, la mitad del equipo ya había utilizado
previamente esta metodología.
Scrum porque fue la metodología recomendada por el maestro; sin embargo, le hicimos
algunas adaptaciones para que se ajustara tanto al tiempo con el que contábamos para esta
primera entrega como a nuestras necesidades como equipo y a las necesidades del
proyecto.
Kanban fue la metodología que añadimos para tener una mejor visualización tanto del flujo
de trabajo como del estado actual del proyecto.

### Roles
En el equipo propusimos tres roles para este proyecto, los cuales son:
- Scrum Master
- Administrador de red
- Desarrollador
Se tomó la decisión de que todos los miembros del equipo desempeñarán cada uno de los
roles propuestos.

<!-- TODO: realizar ajustes pertinentes -->
### *Product Backlog*
Actualmente, el Product Backlog cuenta con un total de 38 actividades; sin embargo, este
número puede incrementarse de acuerdo con los problemas técnicos a los que nos
enfrentemos y las decisiones que tengamos que tomar durante el ciclo de vida del proyecto.
En nuestro Product Backlog contamos con actividades como investigación inicial, instalación
y configuración de máquinas virtuales, creación de Docker, entre otras.

La gestión del *Backlog* la realizamos desde la plataforma *ClickUp*. En el siguiente enlace se
puede encontrar el tablero con todas las actividades del Product Backlog y su estado actual:
[Product Backlog Qubits](https://app.clickup.com/9013738740/v/b/li/901325301305)

<!-- TODO: realizar ajustes pertinentes -->
### *Sprint Backlog*
Hasta el momento hemos realizado dos sprints. El primero se llevó a cabo del 01 de febrero
al 10 de febrero, y el segundo del 10 de febrero al 18 de febrero.
Dentro del Sprint Backlog del primer sprint tenemos actividades como:
- Entendimiento del proyecto
- Investigación de herramientas y algoritmo
- Instalación de Linux
- Configuración de máquinas virtuales
- Instalación de WireGuard
- Configuración de la VPN
- Crear tablero de ClickUp
- Crear presentación de Canva
- Añadir quipo a la presentación de Canva
- Establecer Roles
- Crear repositorio de GitHub

El resto de las actividades se pueden consultar en el siguiente enlace, [Product Backlog Qubits](https://app.clickup.com/9013738740/v/b/li/901325301305)
Qubits, filtrando las actividades que se realizaron entre el 01 de febrero y el 10 de febrero.

### Cronograma
Una de las ventajas de usar *ClickUp* para la gestión de nuestro *Backlog* es que, al ir
agregando tareas y asignando fechas, la plataforma automáticamente genera un
cronograma y un diagrama de Gantt con las actividades.
A continuación, se presentan algunas capturas del cronograma; sin embargo la versión
completa y más actualizada se puede consultar en el siguiente enlace: 
[Diagrama de Gantt Qubits](https://app.clickup.com/9013738740/v/l/li/901325301305)

### Minutas
Las minutas de las reuniones son realizadas por un miembro del equipo, el cual va
alternando en cada sesión, y se estructuran de la siguiente forma:
- Fecha
- Objetivo de la reunión
- Actividades realizadas
- Problemas detectados
- Acuerdos
- Próximos pasos

Al final de cada sesión, el miembro del equipo encargado de la minuta pasa toda la
información en limpio y después la coloca en el siguiente documento de
[Google Docs Minutas Reuniones Qubits](https://docs.google.com/document/d/1unZHQjeayKPoQUWysXpCurW2gVCVXeEnCgpKE55ECkc/edit?pli=1&tab=t.0#heading=h.sqsxan9jf2xa)
en el cual estamos recopilando las minutas de todas nuestras reuniones.

<!-- TODO: Discutir si se agregan las capturas 
### Reuniones
A continuación se presentan algunas capturas tomadas durante nuestras reuniones por
Google Meet como evidencia de la comunicación del equipo.-->

### Topología
Conexiones: Todas las máquinas son capaces de conectarse mediante la VPN a el hub
principal, de la máquina 1 y 2 uno de sus contenedores puede conectarse al hub principal y
mandar ping.

Nodos:
- Máquina 1: esta es el hub principal quien se encarga de recibir las peticiones externas.
- Maquina 2, 3 y 4: son los nodos trabajadores, spokes.

## Conclusión
Se logró levantar correctamente la VPN por medio de WireGuard. Tres máquinas virtuales
de Linux, y una nativa, se conectaron a la red, es decir, la red está conformada por cuatro
nodos, incluyendo el hub. Cada nodo, por medio de Docker Compose y Dockerfile, creó y
ejecutó cuatro contenedores, de los cuales sólo uno de la máquina 1 (el HUB), uno de la
máquina 2 y uno de la máquina 3 fueron conectados a la red.

Después se verificó la conexión entre los nodos conectados a la red. La conexión fue
exitosa. Aunado a esto, se escribió un pequeño programa en Rust. El programa se dividió
en dos partes: servidor y cliente. La parte de servidor se ejecuta en el nodo HUB. Lo que
hace esta parte es “escuchar” si un cliente pide una tarea; de ser así, el servidor envía una
tarea al cliente y espera su respuesta. La parte del cliente se ejecuta en los demás nodos.
Esta parte pide una tarea al servidor; cuando recibe la tarea, la realiza y, finalmente, envía
los resultados al servidor. Este programa funcionó correctamente, lo cual, inferimos, nos
dará una base sólida para implementar el algoritmo que seleccionamos, Cálculo de pi por
método Monte Carlo.

<!-- Pendiente:
Requisitos de software.
- Instrucciones para levantar la VPN (resumidas).
- Instrucciones para desplegar contenedores.
- Instrucciones para compilar y ejecutar el sistema distribuido en Rust.
- Notas importantes y supuestos. -->
