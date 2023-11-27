def selection_sort(input_list: list) -> list:
    length = len(input_list)
    current_min_idx = 0
    for i in range(length):
        print(f"i: {i}")
        for j in range(i + 1, length):
            print(f"j: {j}")
            if input_list[current_min_idx] > input_list[j]:
                current_min_idx = j

        input_list[i], input_list[current_min_idx] = input_list[current_min_idx], input_list[i]
    return input_list


if __name__ == "__main__":
    r = selection_sort([5, 9, 1, 8, 3, 7, 2, 4, 6, 0])
    print(r)
