#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    char username[20];
    int is_admin;
} User;

void print_user(User *user) {
    printf("Username: %s\n", user->username);
    printf("Admin status: %d\n", user->is_admin);
}

int main() {
    User *user = malloc(sizeof(User));  // Allocate memory for a User
    strcpy(user->username, "normal_user");
    user->is_admin = 0;  // Not an admin
    print_user(user);  // Print initial user information
    free(user);  // Free the User struct; `user` is now a dangling pointer

    // Simulate memory reuse
    char *attacker_data = malloc(sizeof(User));
    strcpy(attacker_data, "attacker");
    // Attacker modifies freed memory to escalate privileges
    *(int *)(attacker_data + 20) = 1;  // Overwrite `is_admin` field in the freed User memory

    print_user(user);  // Use-after-free: Accessing freed User struct
    free(attacker_data);
    return 0;
}


