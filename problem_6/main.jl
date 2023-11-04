powsum(x)=sum(collect(1:x).^2)
sum_range(a,b)=(b-a+1)*(a+b)/2
function sum_diff(n)
    csum = 0
    rsum = 0
    for i in n:-1:2
        csum+=i
        rsum+=csum*(i-1)
    end
    return rsum*2
end
sum_diff(100)
# for i in 1:10
    # println(powsum(i))
# end
