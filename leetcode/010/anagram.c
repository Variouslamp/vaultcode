#include <stdio.h>
#include <string.h>
#include <stdbool.h>

bool isAnagram(char* s, char* t) {
    for (int i=0 ; i < strlen(s) ; i++ ) {
        printf("%d", i);
    }
    return 0;
}

int main() {
    char s[] = "Hola";
    char t[] = "Adios";
    
    isAnagram(s, t);

}
