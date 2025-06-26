def quick_sort(nums, low, high):
    if low < high:
        pivot = partition(nums, low, high)
        quick_sort(nums, low, pivot -1)
        quick_sort(nums, pivot + 1, high)

def partition(nums, low, high):
    pivot = nums[high]
    i = low - 1
    for j in range(low, high):
        if nums[j] < pivot:
            i += 1
            nums[i], nums[j] = nums[j], nums[i]
    nums[i + 1], nums[high] = nums[high], nums[i + 1]
    return i + 1


if __name__ == "__main__":
    arr = [5, 9, 1, 8, 3, 7, 2, 4, 6, 0]
    r = quick_sort(arr, 0, len(arr) -1)
    print(arr)
