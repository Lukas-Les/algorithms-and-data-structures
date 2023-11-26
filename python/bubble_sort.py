def buble_sort(input_list: list) -> list:
    length = len(input_list)
    i_loop = 0
    j_loop = 0
    for i in range(length):
        swaped = False
        for j in range(length - i - 1):
            if input_list[j] > input_list[j + 1]:
                input_list[j + 1], input_list[j] = input_list[j], input_list[j + 1]
                swaped = True
            j_loop += 1
        if not swaped:
            break
        i_loop += 1
    print(f"i_loop: {i_loop}")
    print(f"j_loop: {j_loop}")
    return input_list

if __name__ == "__main__":
    r = buble_sort([5, 1, 9, 2, 8, 7, 3, 4, 6, 0])
    print(r)
