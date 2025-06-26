def selection_sort(nums):
    for i in range(len(nums)):
        smallest_idx = i
        for j in range(smallest_idx + 1, len(nums)):
            if nums[j] < nums[smallest_idx]:
                smallest_idx = j
        nums[i], nums[smallest_idx] = nums[smallest_idx], nums[i]
    return nums

if __name__ == "__main__":
    my_list = [5, 9, 1, 8, 3, 7, 2, 4, 6, 0]
    r = selection_sort(my_list)
    print(r)
