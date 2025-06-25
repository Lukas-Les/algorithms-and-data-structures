def merge_sort(nums):
    if len(nums) < 2:
        return nums
    mid = len(nums) // 2
    sorted_left = merge_sort(nums[:mid])
    sorted_right = merge_sort(nums[mid:])
    return merge(sorted_left, sorted_right) 


def merge(first, second):
    final = []
    i, j = 0, 0
    while i < len(first) and j < len(second):
        if first[i] <= second[j]:
            final.append(first[i])
            i += 1
        else:
            final.append(second[j])
            j += 1
    if i < len(first):
        final.extend(first[i:])
    if j < len(second):
        final.extend(second[j:])
    return final


unsorted = [5, 4, 3, 2, 1]
result = merge_sort(unsorted)
print(result)
