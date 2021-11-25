$(() => {
    location.href = '#/blog'
    blogList()
})

function blogList() {
    $.ajax({
        url: '/blog/home',
        type: 'GET',
        success: function (data) {
            $('#main').html(data.data)
        }
    })
}


function blogDetails() {
    $.ajax({
        url: '/blog/details',
        type: 'GET',
        success: function (data) {
            $('#main').html(data.data)
        }
    })
}