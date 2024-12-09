#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BUFFER_SIZE 1024
#define EMPTY 0xFFFF

// Read input from stdin and returns the number of characters read
// The data is stored in the data pointer
// The data pointer should be freed by the caller
size_t get_input(char** data) {
    char buffer[BUFFER_SIZE];

    size_t size = 0;
    size_t s = 0;

    do {
        s = fread(buffer, sizeof(char), BUFFER_SIZE, stdin);
        *data = realloc(*data, size + s);
        memcpy(*data + size, buffer, s);
        size += s;
    } while (s > 0);

    // Removing last \n
    (*data)[--size] = '\0';

    return size;
}

typedef struct node{
    uint16_t value;
    uint16_t size;
    struct node* next;
    struct node* prev;
} node_t;

int main(int argc, char** argv) {
    char* data = NULL;
    size_t data_size = get_input(&data);

    uint64_t score = 0;

    size_t front_reader = 0;
    uint8_t front_count = data[front_reader] - '0';
    size_t back_reader = data_size - 1;
    uint8_t back_count = data[back_reader] - '0';

    size_t write_position = 0;
    while(front_reader < back_reader) {
        if (front_reader % 2) {
            if (front_count) {
                if (back_count) {
                    score += write_position * (back_reader / 2);
                    front_count--;
                    back_count--;
                    write_position++;
                } else {
                    back_reader-=2;
                    back_count = data[back_reader] - '0';
                }
            } else {
                front_reader++;
                front_count = data[front_reader] - '0';
            }
        } else {
            if (front_count) {
                score += write_position * (front_reader / 2);
                front_count--;
                write_position++;
            } else {
                front_reader++;
                front_count = data[front_reader] - '0';
            }
        }
    }

    printf("Part 1: %lu\n", score);

    node_t* head = NULL;

    for (size_t i = 0; i < data_size; i++) {
        node_t* n = (node_t*) malloc(sizeof(node_t));
        n->value = i % 2 == 0 ? i / 2 : EMPTY;
        n->size = data[i] - '0';

        if (head == NULL) {
            head = n;
            n->next = n;
            n->prev = n;
        } else {
            n->next = head;
            n->prev = head->prev;
            head->prev->next = n;
            head->prev = n;
        }
    }

    node_t* back = head->prev;
    while(back != head) {
        if (back->value == EMPTY || back->size == 0) {
            back = back->prev;
            continue;
        }
        for(node_t* front = head; front != back; front = front->next) {
            if (front->value == EMPTY && front->size >= back->size) {
                front->size -= back->size;

                node_t* moved = back;
                back = back->prev;
                moved->prev->next = moved->next;
                moved->next->prev = moved->prev;

                moved->next = front;
                moved->prev = front->prev;

                front->prev->next = moved;
                front->prev = moved;

                node_t* empty = (node_t*) malloc(sizeof(node_t));
                empty->value = EMPTY;
                empty->size = moved->size;

                empty->next = back->next;
                empty->prev = back;
                empty->next->prev = empty;
                empty->prev->next = empty;

                break;
            }
        }
        back = back->prev;

    }

    score = 0;
    uint64_t index = 0;
    node_t* current = head;

    do {
        if (current->value == EMPTY) {
            index += current->size;
        } else {
            for(uint64_t i = 0; i<current->size; i++) {
                score += current->value * index++;
            }
        }
        current = current->next;
    } while (current != head);

    printf("Part 2: %lu\n", score);

    current = head;
    do {
        node_t* next = current->next;
        free(current);
        current = next;
    } while (current != head);
    free(data);
    return 0;
}