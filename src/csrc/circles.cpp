#include <opencv4/opencv2/imgproc.hpp>
#include <opencv4/opencv2/imgcodecs.hpp>
#include <opencv4/opencv2/highgui.hpp>

using namespace std;

//C binding for OpenCV's Houghcircles, so that we can call it easily from Rust.
extern "C" int get_circles_from_img(char* path);

int get_circles_from_img(char* path) {

    cv::Mat img, gray;
    img = cv::imread(path, 0);
    //cv::cvtColor(img, gray, cv::COLOR_BGR2GRAY);
    cv::medianBlur(img,img,1);
    vector<cv::Vec3f> circles;
    cv::HoughCircles( //TODO: move parameters out
        img,
        circles,
        cv::HOUGH_GRADIENT, //Below comments are wrong if changed from gradient.
        1.0,            //Resolution
        10.0,           //Min distance between centers
        400.0,          //Canny edge detector threshold
        30.0,           //"Perfectness" higher is more perfect
        10,             //Min circle radius
        50
    );

    //Temp testing
    for( size_t i = 0; i < circles.size(); i++ )
    {
        cv::Point center(cvRound(circles[i][0]), cvRound(circles[i][1]));
        int radius = cvRound(circles[i][2]);

        circle( img, center, 2, cv::Scalar(255,255,255), 3, cv::LINE_8, 0 );
        circle( img, center, radius, cv::Scalar(255,255,255), 2, cv::LINE_8, 0 );
    }
    cv::namedWindow( "circles", 1 );
    cv::imshow( "circles", img );
    cv::waitKey(0);

    //Testing end

    return 0; 
}