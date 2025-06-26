def bubble_sort(input_list: list) -> list:
    """
    Sorts a list of elements using the Bubble Sort algorithm.

    Args:
        input_list: The list to be sorted.

    Returns:
        The sorted list.
    """
    length = len(input_list)
    for i in range(length):
        swapped = False
        for j in range(length - i - 1):
            if input_list[j] > input_list[j + 1]:
                input_list[j + 1], input_list[j] = input_list[j], input_list[j + 1]
                swapped = True
        if not swapped:
            break
    return input_list

if __name__ == "__main__":
    r = bubble_sort([5, 1, 9, 2, 8, 7, 3, 4, 6, 0])
    print(r)
