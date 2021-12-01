/*!
* YesTop(jQuery GoToTop)
* version: 1.1.2
* Copyright (c) 2015 HoverTree
* http://hovertree.com
* http://hovertree.com/texiao/yestop/
*/
(function ($) {
    $.fn.yestop = function (options) {

        var settings = $.extend({
            yes_position: 'fixed',
            yes_width: '48px',
            yes_height: '48px',
            yes_image: '',
            yes_hoverImage: '',
            yes_length: '268',
            yes_time: 500,
            yes_bottom: '50px',
            yes_right: "50px",
            yes_top: "",
            yes_left: "",
            yes_title: "Go Top",
            yes_opacity: "0.8",
            yes_hoverOpacity:"1",
            yes_radius: "0%",
            yes_html: "",
            yes_hoverHtml: "",
            yes_fontSize:"12px",
            yes_lineHeight: "48px",
            yes_backColor: "transparent"
        }, options);

        settings.yes_image = "url(" + settings.yes_image + ")";


        var h_yesObj;
        if($(this).length<1)//
        {
            if ($("#yesTopHovertree").length < 1) {
                $("<div id='yesTopHovertree'></div>").appendTo("body");
            }
            h_yesObj = $("#yesTopHovertree");
        }
        else {
            h_yesObj = $(this);
        }

        h_yesObj.css({
            "width": settings.yes_width, "height": settings.yes_height
            , "cursor": "pointer", "border-radius": settings.yes_radius
            , "position": settings.yes_position 
            , "opacity": settings.yes_opacity, "background-image": settings.yes_image
            , "text-align": "center", "line-height": settings.yes_lineHeight
            ,"background-color":settings.yes_backColor,"font-size":settings.yes_fontSize
        })

        if (settings.yes_html != "")
        { h_yesObj.html(settings.yes_html) }

        if (settings.yes_hoverHtml != "")
        {
            h_yesObj.hover(function () { h_yesObj.html(settings.yes_hoverHtml) }, function () { h_yesObj.html(settings.yes_html) })
        }

        if (settings.yes_position == "fixed") {
            if (settings.yes_top == "") {
                h_yesObj.css({ "bottom": settings.yes_bottom })
            }
            else { h_yesObj.css({ "top": settings.yes_top }) }
            if(settings.yes_left=="")
            { h_yesObj.css({ "right": settings.yes_right }) }
            else { h_yesObj.css({ "left": settings.yes_left }) }
        }

        h_yesObj.hover(
          function () {
              h_yesObj.css({ "opacity": settings.yes_hoverOpacity })
          },
          function () {
              h_yesObj.css({ "opacity": settings.yes_opacity })
          }
        )
        

        if (settings.yes_hoverImage != "")
        {
            settings.yes_hoverImage = "url(" + settings.yes_hoverImage + ")"
            h_yesObj.hover(
                function () { h_yesObj.css({ "background-image": settings.yes_hoverImage });}
                , function () { h_yesObj.css({ "background-image": settings.yes_image }) });
        }

        h_yesObj.attr("title", settings.yes_title);

        if (settings.yes_length > 0)
            h_yesObj.hide();
        else { h_yesObj.show();}

        $(window).scroll(function () {
            if ($(window).scrollTop() > settings.yes_length) {
                h_yesObj.fadeIn(100);
            }
            else {
                h_yesObj.fadeOut(100);
            }
        });

        h_yesObj.on("click", function () {$('html,body').animate({ scrollTop: '0px' }, settings.yes_time); return false;  })

    }
}(jQuery));