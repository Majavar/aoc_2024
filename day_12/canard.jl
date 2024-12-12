# Define a function to be applied to each element
function print_element(x)
    println(x)
end

# Create a collection
collection = 2:5

# Apply the function to each element in the collection
foreach(print_element, collection)