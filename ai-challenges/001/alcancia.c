#include <stdio.h>
#include <stdlib.h>

// ------------------------------------------------------------------
// Funcion la cual permite hacer un input de un numero
// unicamente ingresando el texto para que el usuario entienda

int int_input(char *text, bool *error){
	char buffer[10000];
	char *fin;

	printf("%s\n", text);
	fgets(buffer, sizeof(buffer), stdin);

	long number = strtol(buffer, &fin, 10);
	if (*fin != '\n') {
		*error = true;
	}
	else {
		*error = false;
	}
	return number;
}


// ------------------------------------------------------------------
// funcion que hace utilizacion del input para hacer un bucle
// de error cuando el usuario ingrese un valor no permitido sse repita
// la pregunta con un aviso


int validador(char *text){
	bool error = false;
	int numero;
	while (true){
		numero = int_input(text, &error);
		if (error == false){
			break;
		}
		printf("Error valor ingresado invalido")
	}
	return numero;
}


// ------------------------------------------------------------------
// Funcion main en la que se ensamblan y hacen uso los modulos
// anteriores

int main(){
	int total_value = 0;
	int objetivo = validador("Ingresar su meta de ahorro: ");
	int sumar;

	while (total_value < objetivo) {
		sumar = validador("cuanto le quiere meter?: ");
		total_value += sumar;
		int falta = objetivo - total_value;
		printf("La alcancia tiene %i COP faltan %i COP \n\n", total_value, falta);
	}
	printf("MUY BIEN TERMINASTE DE AHORRAR");
	return 0;
}

