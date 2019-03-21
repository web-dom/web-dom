(function (factory) {
  typeof define === 'function' && define.amd ? define(factory) :
  factory();
}(function () { 'use strict';

  function allocator() {
    let allocations = [];
    let empty = [];
    return {
      //allocate
      a:function(value){
        let i = allocations.length;
        if(empty.length > 0) {
          i = empty.pop();
        }
        allocations[i] = value;
        return i;
      },
      //release
      r(handle){
          allocations[handle] = undefined;
          empty.push(handle);
      },
      //get
      g(handle){
        if(handle < 0){
          return undefined;
        }
        let ret =  allocations[handle];
        if(!ret){
          console.error(`Asked for ${handle} after it was released.`);
        }
        return ret;
      }
    }
  }

  // THIS FILE IS GENERATED FROM tools/generate_webidl.js

  function createWebIDLContext() {
    let ALLOCATOR = allocator();
    const webidl = {
      global_debugger: function() {
        debugger;
      },

      global_get_window: function() {
        return ALLOCATOR.a(window);
      },

      global_release: function(handle) {
        ALLOCATOR.r(handle);
      },
      global_create_event_listener: function() {
        let handle = ALLOCATOR.a(e => this.executeCallback(handle, e, ALLOCATOR));
        return handle;
      },
      global_get_property: function(handle, name) {
        let o = ALLOCATOR.g(handle);
        let p = o[this.s(name)];
        if (typeof p == "string") {
          return this.ms(p);
        } else if (typeof p == "boolean") {
          return p ? 1 : 0;
        } else if (typeof p == "number") {
          return p;
        }
        return ALLOCATOR.a(p);
      },
      date_now: function() {
        return Date.now();
      },
      date_now_seconds: function() {
        return Date.now() / 1000;
      },
      date_get_timezone_offset: function() {
        return new Date().getTimezoneOffset();
      },
      math_random: function() {
        return Math.random();
      },
      math_random_n: function(n) {
        return Math.floor(Math.random() * n);
      },
      customelement_attach_shadow: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.attachShadow({ mode: "open" }));
      },
      customelement_define: function(componentName) {
        componentName = this.s(componentName);
        let createElement = this.elementCreated;
        window.setTimeout(() => {
          customElements.define(
            componentName,
            class extends HTMLElement {
              constructor() {
                super();
                var e = new CustomEvent("customelementcreated", {
                  detail: ALLOCATOR.a(this)
                });
                window.dispatchEvent(e);
              }
              connectedCallback() {
                var e = new CustomEvent("connected");
                this.dispatchEvent(e);
              }
              disconnectedCallback() {
                debugger;
                var e = new CustomEvent("disconnected");
                this.dispatchEvent(e);
              }
              adoptedCallback() {
                var e = new CustomEvent("adopted");
                this.dispatchEvent(e);
              }
              attributeChangedCallback(name, oldValue, value) {
                var e = new CustomEvent("attributechanged", {
                  detail: { name, oldValue, value }
                });
                this.dispatchEvent(e);
              }
            }
          );
        }, 1);
      },
      customelement_define_with_attributes: function(componentName, attributes) {
        componentName = this.s(componentName);
        attributes = this.s(attributes);
        let createElement = this.elementCreated;
        let observedAttributes = attributes.split(",").map(x => x.trim());
        window.setTimeout(() => {
          customElements.define(
            componentName,
            class extends HTMLElement {
              constructor() {
                super();
                var e = new CustomEvent("customelementcreated", {
                  detail: ALLOCATOR.a(this)
                });
                window.dispatchEvent(e);
              }
              static get observedAttributes() {
                return observedAttributes;
              }
              connectedCallback() {
                var e = new CustomEvent("connected");
                this.dispatchEvent(e);
              }
              disconnectedCallback() {
                debugger;
                var e = new CustomEvent("disconnected");
                this.dispatchEvent(e);
              }
              adoptedCallback() {
                var e = new CustomEvent("adopted");
                this.dispatchEvent(e);
              }
              attributeChangedCallback(name, oldValue, value) {
                var e = new CustomEvent("attributechanged", {
                  detail: { name, oldValue, value }
                });
                this.dispatchEvent(e);
              }
            }
          );
        }, 1);
      },

      WasmWorker_onWorkerLoaded: function(instance, listener) {
        let _instance = ALLOCATOR.g(instance);
        let _listener = ALLOCATOR.g(listener);
        if (_instance.loaded) {
          _listener(
            new CustomEvent("load", { detail: { id: _instance.workerId } })
          );
        } else {
          _instance.addEventListener("load", _listener);
        }
      },
      WasmWorker_onWorkerMessage: function(instance, listener) {
        let _instance = ALLOCATOR.g(instance);
        let _listener = ALLOCATOR.g(listener);
        _instance.addEventListener("message", _listener);
      },
      WasmWorker_sendWorkerMessage: function(instance, start, len) {
        let _instance = ALLOCATOR.g(instance);
        const data = new Uint8Array(this.memory.buffer);
        _instance.sendMessage(data.subarray(start, start + len));
      },
      WasmWorkerLoadEvent_getWorkerId: function(ev) {
        let e = ALLOCATOR.g(ev);
        return e.detail.id;
      },
      WasmWorkerMessageEvent_get_length: function(ev) {
        let e = ALLOCATOR.g(ev);
        return e.detail.length;
      },
      WasmWorkerMessageEvent_get_data: function(ev) {
        let e = ALLOCATOR.g(ev);
        let start = this.m(e.length);
        const data = new Uint8Array(this.memory.buffer);
        data.set(e.detail, start);
        return start;
      },

      //TODO: get rid of one day when this isn't required by tinygo
      io_get_stdout: function() {
        return 1;
      },

      //TODO: get rid of one day when this isn't required by tinygo
      resource_write: function() {},

      canvasrenderingcontext2d_get_canvas: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.canvas);
      },

      canvasrenderingcontext2d_set_canvas: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.canvas = ALLOCATOR.g(handle);
      },

      canvasrenderingcontext2d_draw_window: function(
        instance,
        window,
        x,
        y,
        w,
        h,
        bg_color,
        flags
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _window = ALLOCATOR.g(window);
        let _x = x;
        let _y = y;
        let _w = w;
        let _h = h;
        let _bg_color = this.s(bg_color);
        let _flags = flags;
        _instance.drawWindow(_window, _x, _y, _w, _h, _bg_color, _flags);
      },

      canvasrenderingcontext2d_demote: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.demote();
      },

      canvasrenderingcontext2d_save: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.save();
      },

      canvasrenderingcontext2d_restore: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.restore();
      },

      canvasrenderingcontext2d_scale: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.scale(_x, _y);
      },

      canvasrenderingcontext2d_rotate: function(instance, angle) {
        let _instance = ALLOCATOR.g(instance);
        let _angle = angle;
        _instance.rotate(_angle);
      },

      canvasrenderingcontext2d_translate: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.translate(_x, _y);
      },

      canvasrenderingcontext2d_transform: function(instance, a, b, c, d, e, f) {
        let _instance = ALLOCATOR.g(instance);
        let _a = a;
        let _b = b;
        let _c = c;
        let _d = d;
        let _e = e;
        let _f = f;
        _instance.transform(_a, _b, _c, _d, _e, _f);
      },

      canvasrenderingcontext2d_set_transform: function(
        instance,
        a,
        b,
        c,
        d,
        e,
        f
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _a = a;
        let _b = b;
        let _c = c;
        let _d = d;
        let _e = e;
        let _f = f;
        _instance.setTransform(_a, _b, _c, _d, _e, _f);
      },

      canvasrenderingcontext2d_reset_transform: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.resetTransform();
      },

      canvasrenderingcontext2d_get_global_alpha: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.globalAlpha;
      },

      canvasrenderingcontext2d_set_global_alpha: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.globalAlpha = val;
      },

      canvasrenderingcontext2d_get_global_composite_operation: function(
        instance
      ) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.globalCompositeOperation);
      },

      canvasrenderingcontext2d_set_global_composite_operation: function(
        instance,
        str
      ) {
        let _instance = ALLOCATOR.g(instance);
        _instance.globalCompositeOperation = this.s(str);
      },

      canvasrenderingcontext2d_get_image_smoothing_enabled: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.imageSmoothingEnabled;
      },

      canvasrenderingcontext2d_set_image_smoothing_enabled: function(
        instance,
        val
      ) {
        let _instance = ALLOCATOR.g(instance);
        _instance.imageSmoothingEnabled = val;
      },

      canvasrenderingcontext2d_get_stroke_style: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.strokeStyle);
      },

      canvasrenderingcontext2d_set_stroke_style: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.strokeStyle = this.s(str);
      },

      canvasrenderingcontext2d_get_fill_style: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.fillStyle);
      },

      canvasrenderingcontext2d_set_fill_style: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.fillStyle = this.s(str);
      },

      canvasrenderingcontext2d_create_linear_gradient: function(
        instance,
        x0,
        y0,
        x1,
        y1
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _x0 = x0;
        let _y0 = y0;
        let _x1 = x1;
        let _y1 = y1;
        return ALLOCATOR.a(_instance.createLinearGradient(_x0, _y0, _x1, _y1));
      },

      canvasrenderingcontext2d_create_radial_gradient: function(
        instance,
        x0,
        y0,
        r0,
        x1,
        y1,
        r1
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _x0 = x0;
        let _y0 = y0;
        let _r0 = r0;
        let _x1 = x1;
        let _y1 = y1;
        let _r1 = r1;
        return ALLOCATOR.a(
          _instance.createRadialGradient(_x0, _y0, _r0, _x1, _y1, _r1)
        );
      },

      canvasrenderingcontext2d_create_pattern: function(
        instance,
        image,
        repetition
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _image = ALLOCATOR.g(image);
        let _repetition = this.s(repetition);
        return ALLOCATOR.a(_instance.createPattern(_image, _repetition));
      },

      canvasrenderingcontext2d_get_shadow_offset_x: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.shadowOffsetX;
      },

      canvasrenderingcontext2d_set_shadow_offset_x: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.shadowOffsetX = val;
      },

      canvasrenderingcontext2d_get_shadow_offset_y: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.shadowOffsetY;
      },

      canvasrenderingcontext2d_set_shadow_offset_y: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.shadowOffsetY = val;
      },

      canvasrenderingcontext2d_get_shadow_blur: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.shadowBlur;
      },

      canvasrenderingcontext2d_set_shadow_blur: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.shadowBlur = val;
      },

      canvasrenderingcontext2d_get_shadow_color: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.shadowColor);
      },

      canvasrenderingcontext2d_set_shadow_color: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.shadowColor = this.s(str);
      },

      canvasrenderingcontext2d_get_filter: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.filter);
      },

      canvasrenderingcontext2d_set_filter: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.filter = this.s(str);
      },

      canvasrenderingcontext2d_clear_rect: function(instance, x, y, w, h) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        let _w = w;
        let _h = h;
        _instance.clearRect(_x, _y, _w, _h);
      },

      canvasrenderingcontext2d_fill_rect: function(instance, x, y, w, h) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        let _w = w;
        let _h = h;
        _instance.fillRect(_x, _y, _w, _h);
      },

      canvasrenderingcontext2d_stroke_rect: function(instance, x, y, w, h) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        let _w = w;
        let _h = h;
        _instance.strokeRect(_x, _y, _w, _h);
      },

      canvasrenderingcontext2d_begin_path: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.beginPath();
      },

      canvasrenderingcontext2d_fill: function(instance, path, winding) {
        let _instance = ALLOCATOR.g(instance);
        let _path = ALLOCATOR.g(path);
        let _winding = ALLOCATOR.g(winding);
        _instance.fill(_path, _winding);
      },

      canvasrenderingcontext2d_stroke: function(instance, path) {
        let _instance = ALLOCATOR.g(instance);
        let _path = ALLOCATOR.g(path);
        _instance.stroke(_path);
      },

      canvasrenderingcontext2d_clip: function(instance, path, winding) {
        let _instance = ALLOCATOR.g(instance);
        let _path = ALLOCATOR.g(path);
        let _winding = ALLOCATOR.g(winding);
        _instance.clip(_path, _winding);
      },

      canvasrenderingcontext2d_is_point_in_path: function(
        instance,
        path,
        x,
        y,
        winding
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _path = ALLOCATOR.g(path);
        let _x = x;
        let _y = y;
        let _winding = ALLOCATOR.g(winding);
        return ALLOCATOR.a(_instance.isPointInPath(_path, _x, _y, _winding));
      },

      canvasrenderingcontext2d_is_point_in_stroke: function(
        instance,
        path,
        x,
        y
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _path = ALLOCATOR.g(path);
        let _x = x;
        let _y = y;
        return ALLOCATOR.a(_instance.isPointInStroke(_path, _x, _y));
      },

      canvasrenderingcontext2d_draw_focus_if_needed: function(instance, element) {
        let _instance = ALLOCATOR.g(instance);
        let _element = ALLOCATOR.g(element);
        _instance.drawFocusIfNeeded(_element);
      },

      canvasrenderingcontext2d_draw_custom_focus_ring: function(
        instance,
        element
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _element = ALLOCATOR.g(element);
        return ALLOCATOR.a(_instance.drawCustomFocusRing(_element));
      },

      canvasrenderingcontext2d_fill_text: function(
        instance,
        text,
        x,
        y,
        max_width
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _text = this.s(text);
        let _x = x;
        let _y = y;
        let _max_width = max_width;
        _instance.fillText(_text, _x, _y, _max_width);
      },

      canvasrenderingcontext2d_stroke_text: function(
        instance,
        text,
        x,
        y,
        max_width
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _text = this.s(text);
        let _x = x;
        let _y = y;
        let _max_width = max_width;
        _instance.strokeText(_text, _x, _y, _max_width);
      },

      canvasrenderingcontext2d_measure_text: function(instance, text) {
        let _instance = ALLOCATOR.g(instance);
        let _text = this.s(text);
        return ALLOCATOR.a(_instance.measureText(_text));
      },

      canvasrenderingcontext2d_draw_image: function(
        instance,
        image,
        sx,
        sy,
        sw,
        sh,
        dx,
        dy,
        dw,
        dh
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _image = ALLOCATOR.g(image);
        let _sx = sx;
        let _sy = sy;
        let _sw = sw;
        let _sh = sh;
        let _dx = dx;
        let _dy = dy;
        let _dw = dw;
        let _dh = dh;
        _instance.drawImage(_image, _sx, _sy, _sw, _sh, _dx, _dy, _dw, _dh);
      },

      canvasrenderingcontext2d_create_image_data: function(instance, sw, sh) {
        let _instance = ALLOCATOR.g(instance);
        let _sw = sw;
        let _sh = sh;
        return ALLOCATOR.a(_instance.createImageData(_sw, _sh));
      },

      canvasrenderingcontext2d_get_image_data: function(
        instance,
        sx,
        sy,
        sw,
        sh
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _sx = sx;
        let _sy = sy;
        let _sw = sw;
        let _sh = sh;
        return ALLOCATOR.a(_instance.getImageData(_sx, _sy, _sw, _sh));
      },

      canvasrenderingcontext2d_put_image_data: function(
        instance,
        imagedata,
        dx,
        dy,
        dirty_x,
        dirty_y,
        dirty_width,
        dirty_height
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _imagedata = ALLOCATOR.g(imagedata);
        let _dx = dx;
        let _dy = dy;
        let _dirty_x = dirty_x;
        let _dirty_y = dirty_y;
        let _dirty_width = dirty_width;
        let _dirty_height = dirty_height;
        _instance.putImageData(
          _imagedata,
          _dx,
          _dy,
          _dirty_x,
          _dirty_y,
          _dirty_width,
          _dirty_height
        );
      },

      canvasrenderingcontext2d_get_line_width: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.lineWidth;
      },

      canvasrenderingcontext2d_set_line_width: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.lineWidth = val;
      },

      canvasrenderingcontext2d_get_line_cap: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.lineCap);
      },

      canvasrenderingcontext2d_set_line_cap: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.lineCap = this.s(str);
      },

      canvasrenderingcontext2d_get_line_join: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.lineJoin);
      },

      canvasrenderingcontext2d_set_line_join: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.lineJoin = this.s(str);
      },

      canvasrenderingcontext2d_get_miter_limit: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.miterLimit;
      },

      canvasrenderingcontext2d_set_miter_limit: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.miterLimit = val;
      },

      canvasrenderingcontext2d_set_line_dash: function(instance, segments) {
        let _instance = ALLOCATOR.g(instance);
        let _segments = ALLOCATOR.g(segments);
        _instance.setLineDash(_segments);
      },

      canvasrenderingcontext2d_get_line_dash: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getLineDash());
      },

      canvasrenderingcontext2d_get_line_dash_offset: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.lineDashOffset;
      },

      canvasrenderingcontext2d_set_line_dash_offset: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.lineDashOffset = val;
      },

      canvasrenderingcontext2d_get_font: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.font);
      },

      canvasrenderingcontext2d_set_font: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.font = this.s(str);
      },

      canvasrenderingcontext2d_get_text_align: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.textAlign);
      },

      canvasrenderingcontext2d_set_text_align: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.textAlign = this.s(str);
      },

      canvasrenderingcontext2d_get_text_baseline: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.textBaseline);
      },

      canvasrenderingcontext2d_set_text_baseline: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.textBaseline = this.s(str);
      },

      canvasrenderingcontext2d_close_path: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.closePath();
      },

      canvasrenderingcontext2d_move_to: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.moveTo(_x, _y);
      },

      canvasrenderingcontext2d_line_to: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.lineTo(_x, _y);
      },

      canvasrenderingcontext2d_quadratic_curve_to: function(
        instance,
        cpx,
        cpy,
        x,
        y
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _cpx = cpx;
        let _cpy = cpy;
        let _x = x;
        let _y = y;
        _instance.quadraticCurveTo(_cpx, _cpy, _x, _y);
      },

      canvasrenderingcontext2d_bezier_curve_to: function(
        instance,
        cp1x,
        cp1y,
        cp2x,
        cp2y,
        x,
        y
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _cp1x = cp1x;
        let _cp1y = cp1y;
        let _cp2x = cp2x;
        let _cp2y = cp2y;
        let _x = x;
        let _y = y;
        _instance.bezierCurveTo(_cp1x, _cp1y, _cp2x, _cp2y, _x, _y);
      },

      canvasrenderingcontext2d_arc_to: function(
        instance,
        x1,
        y1,
        x2,
        y2,
        radius
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _x1 = x1;
        let _y1 = y1;
        let _x2 = x2;
        let _y2 = y2;
        let _radius = radius;
        _instance.arcTo(_x1, _y1, _x2, _y2, _radius);
      },

      canvasrenderingcontext2d_rect: function(instance, x, y, w, h) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        let _w = w;
        let _h = h;
        _instance.rect(_x, _y, _w, _h);
      },

      canvasrenderingcontext2d_arc: function(
        instance,
        x,
        y,
        radius,
        start_angle,
        end_angle,
        anticlockwise
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        let _radius = radius;
        let _start_angle = start_angle;
        let _end_angle = end_angle;
        let _anticlockwise = anticlockwise;
        _instance.arc(_x, _y, _radius, _start_angle, _end_angle, _anticlockwise);
      },

      canvasrenderingcontext2d_ellipse: function(
        instance,
        x,
        y,
        radius_x,
        radius_y,
        rotation,
        start_angle,
        end_angle,
        anticlockwise
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        let _radius_x = radius_x;
        let _radius_y = radius_y;
        let _rotation = rotation;
        let _start_angle = start_angle;
        let _end_angle = end_angle;
        let _anticlockwise = anticlockwise;
        _instance.ellipse(
          _x,
          _y,
          _radius_x,
          _radius_y,
          _rotation,
          _start_angle,
          _end_angle,
          _anticlockwise
        );
      },

      canvasrenderingcontext2d_add_hit_region: function(instance, options) {
        let _instance = ALLOCATOR.g(instance);
        let _options = ALLOCATOR.g(options);
        _instance.addHitRegion(_options);
      },

      canvasrenderingcontext2d_remove_hit_region: function(instance, id) {
        let _instance = ALLOCATOR.g(instance);
        let _id = this.s(id);
        _instance.removeHitRegion(_id);
      },

      canvasrenderingcontext2d_clear_hit_regions: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.clearHitRegions();
      },

      canvasgradient_add_color_stop: function(instance, offset, color) {
        let _instance = ALLOCATOR.g(instance);
        let _offset = offset;
        let _color = this.s(color);
        _instance.addColorStop(_offset, _color);
      },

      canvaspattern_set_transform: function(instance, matrix) {
        let _instance = ALLOCATOR.g(instance);
        let _matrix = ALLOCATOR.g(matrix);
        _instance.setTransform(_matrix);
      },

      textmetrics_get_width: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.width;
      },

      textmetrics_set_width: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.width = val;
      },

      path2d_add_path: function(instance, path, transformation) {
        let _instance = ALLOCATOR.g(instance);
        let _path = ALLOCATOR.g(path);
        let _transformation = ALLOCATOR.g(transformation);
        _instance.addPath(_path, _transformation);
      },

      console_assert: function(condition, message) {
        let _condition = condition;
        let _message = this.s(message);
        console.assert(_condition, _message);
      },

      console_clear: function() {
        console.clear();
      },

      console_count: function(label) {
        let _label = this.s(label);
        console.count(_label);
      },

      console_count_reset: function(label) {
        let _label = this.s(label);
        console.countReset(_label);
      },

      console_debug: function(message) {
        let _message = this.s(message);
        console.debug(_message);
      },

      console_error: function(message) {
        let _message = this.s(message);
        console.error(_message);
      },

      console_info: function(message) {
        let _message = this.s(message);
        console.info(_message);
      },

      console_log: function(message) {
        let _message = this.s(message);
        console.log(_message);
      },

      console_table: function(message) {
        let _message = this.s(message);
        console.table(_message);
      },

      console_trace: function(message) {
        let _message = this.s(message);
        console.trace(_message);
      },

      console_warn: function(message) {
        let _message = this.s(message);
        console.warn(_message);
      },

      console_dir: function(message) {
        let _message = this.s(message);
        console.dir(_message);
      },

      console_dirxml: function(message) {
        let _message = this.s(message);
        console.dirxml(_message);
      },

      console_group: function(message) {
        let _message = this.s(message);
        console.group(_message);
      },

      console_group_collapsed: function(message) {
        let _message = this.s(message);
        console.groupCollapsed(_message);
      },

      console_group_end: function() {
        console.groupEnd();
      },

      console_time: function(label) {
        let _label = this.s(label);
        console.time(_label);
      },

      console_time_log: function(label, message) {
        let _label = this.s(label);
        let _message = this.s(message);
        console.timeLog(_label, _message);
      },

      console_time_end: function(label) {
        let _label = this.s(label);
        console.timeEnd(_label);
      },

      console_exception: function(message) {
        let _message = this.s(message);
        console.exception(_message);
      },

      console_time_stamp: function(message) {
        let _message = this.s(message);
        console.timeStamp(_message);
      },

      console_profile: function(message) {
        let _message = this.s(message);
        console.profile(_message);
      },

      console_profile_end: function(message) {
        let _message = this.s(message);
        console.profileEnd(_message);
      },

      consoleinstance_assert: function(instance, condition, message) {
        let _instance = ALLOCATOR.g(instance);
        let _condition = condition;
        let _message = this.s(message);
        _instance.assert(_condition, _message);
      },

      consoleinstance_clear: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.clear();
      },

      consoleinstance_count: function(instance, label) {
        let _instance = ALLOCATOR.g(instance);
        let _label = this.s(label);
        _instance.count(_label);
      },

      consoleinstance_count_reset: function(instance, label) {
        let _instance = ALLOCATOR.g(instance);
        let _label = this.s(label);
        _instance.countReset(_label);
      },

      consoleinstance_debug: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.debug(_message);
      },

      consoleinstance_error: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.error(_message);
      },

      consoleinstance_info: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.info(_message);
      },

      consoleinstance_log: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.log(_message);
      },

      consoleinstance_table: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.table(_message);
      },

      consoleinstance_trace: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.trace(_message);
      },

      consoleinstance_warn: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.warn(_message);
      },

      consoleinstance_dir: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.dir(_message);
      },

      consoleinstance_dirxml: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.dirxml(_message);
      },

      consoleinstance_group: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.group(_message);
      },

      consoleinstance_group_collapsed: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.groupCollapsed(_message);
      },

      consoleinstance_group_end: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.groupEnd();
      },

      consoleinstance_time: function(instance, label) {
        let _instance = ALLOCATOR.g(instance);
        let _label = this.s(label);
        _instance.time(_label);
      },

      consoleinstance_time_log: function(instance, label, message) {
        let _instance = ALLOCATOR.g(instance);
        let _label = this.s(label);
        let _message = this.s(message);
        _instance.timeLog(_label, _message);
      },

      consoleinstance_time_end: function(instance, label) {
        let _instance = ALLOCATOR.g(instance);
        let _label = this.s(label);
        _instance.timeEnd(_label);
      },

      consoleinstance_exception: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.exception(_message);
      },

      consoleinstance_time_stamp: function(instance, data) {
        let _instance = ALLOCATOR.g(instance);
        let _data = data;
        _instance.timeStamp(_data);
      },

      consoleinstance_profile: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.profile(_message);
      },

      consoleinstance_profile_end: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.profileEnd(_message);
      },

      consoleinstance_report_for_service_worker_scope: function(
        instance,
        scope,
        message,
        filename,
        line_number,
        column_number,
        level
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _scope = this.s(scope);
        let _message = this.s(message);
        let _filename = this.s(filename);
        let _line_number = line_number;
        let _column_number = column_number;
        let _level = ALLOCATOR.g(level);
        _instance.reportForServiceWorkerScope(
          _scope,
          _message,
          _filename,
          _line_number,
          _column_number,
          _level
        );
      },

      document_get_implementation: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.implementation);
      },

      document_set_implementation: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.implementation = ALLOCATOR.g(handle);
      },

      document_get_url: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.URL);
      },

      document_set_url: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.URL = this.s(str);
      },

      document_get_document_uri: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.documentURI);
      },

      document_set_document_uri: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.documentURI = this.s(str);
      },

      document_get_compat_mode: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.compatMode);
      },

      document_set_compat_mode: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.compatMode = this.s(str);
      },

      document_get_character_set: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.characterSet);
      },

      document_set_character_set: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.characterSet = this.s(str);
      },

      document_get_charset: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.charset);
      },

      document_set_charset: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.charset = this.s(str);
      },

      document_get_input_encoding: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.inputEncoding);
      },

      document_set_input_encoding: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.inputEncoding = this.s(str);
      },

      document_get_content_type: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.contentType);
      },

      document_set_content_type: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.contentType = this.s(str);
      },

      document_get_doctype: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.doctype);
      },

      document_set_doctype: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.doctype = ALLOCATOR.g(handle);
      },

      document_get_document_element: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.documentElement);
      },

      document_set_document_element: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.documentElement = ALLOCATOR.g(handle);
      },

      document_get_elements_by_tag_name: function(instance, local_name) {
        let _instance = ALLOCATOR.g(instance);
        let _local_name = this.s(local_name);
        return ALLOCATOR.a(_instance.getElementsByTagName(_local_name));
      },

      document_get_elements_by_tag_name_n_s: function(
        instance,
        namespace,
        local_name
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _namespace = this.s(namespace);
        let _local_name = this.s(local_name);
        return ALLOCATOR.a(
          _instance.getElementsByTagNameNS(_namespace, _local_name)
        );
      },

      document_get_elements_by_class_name: function(instance, class_names) {
        let _instance = ALLOCATOR.g(instance);
        let _class_names = this.s(class_names);
        return ALLOCATOR.a(_instance.getElementsByClassName(_class_names));
      },

      document_get_element_by_id: function(instance, element_id) {
        let _instance = ALLOCATOR.g(instance);
        let _element_id = this.s(element_id);
        return ALLOCATOR.a(_instance.getElementById(_element_id));
      },

      document_create_element: function(instance, local_name, options) {
        let _instance = ALLOCATOR.g(instance);
        let _local_name = this.s(local_name);
        let _options = ALLOCATOR.g(options);
        return ALLOCATOR.a(_instance.createElement(_local_name, _options));
      },

      document_create_element_n_s: function(
        instance,
        namespace,
        qualified_name,
        options
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _namespace = this.s(namespace);
        let _qualified_name = this.s(qualified_name);
        let _options = ALLOCATOR.g(options);
        return ALLOCATOR.a(
          _instance.createElementNS(_namespace, _qualified_name, _options)
        );
      },

      document_create_document_fragment: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.createDocumentFragment());
      },

      document_create_text_node: function(instance, data) {
        let _instance = ALLOCATOR.g(instance);
        let _data = this.s(data);
        return ALLOCATOR.a(_instance.createTextNode(_data));
      },

      document_create_comment: function(instance, data) {
        let _instance = ALLOCATOR.g(instance);
        let _data = this.s(data);
        return ALLOCATOR.a(_instance.createComment(_data));
      },

      document_create_processing_instruction: function(instance, target, data) {
        let _instance = ALLOCATOR.g(instance);
        let _target = this.s(target);
        let _data = this.s(data);
        return ALLOCATOR.a(_instance.createProcessingInstruction(_target, _data));
      },

      document_import_node: function(instance, node, deep) {
        let _instance = ALLOCATOR.g(instance);
        let _node = ALLOCATOR.g(node);
        let _deep = deep;
        return ALLOCATOR.a(_instance.importNode(_node, _deep));
      },

      document_adopt_node: function(instance, node) {
        let _instance = ALLOCATOR.g(instance);
        let _node = ALLOCATOR.g(node);
        return ALLOCATOR.a(_instance.adoptNode(_node));
      },

      document_create_event: function(instance, name) {
        let _instance = ALLOCATOR.g(instance);
        let _name = this.s(name);
        return ALLOCATOR.a(_instance.createEvent(_name));
      },

      document_create_range: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.createRange());
      },

      document_create_node_iterator: function(
        instance,
        root,
        what_to_show,
        filter
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _root = ALLOCATOR.g(root);
        let _what_to_show = what_to_show;
        let _filter = ALLOCATOR.g(filter);
        return ALLOCATOR.a(
          _instance.createNodeIterator(_root, _what_to_show, _filter)
        );
      },

      document_create_tree_walker: function(
        instance,
        root,
        what_to_show,
        filter
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _root = ALLOCATOR.g(root);
        let _what_to_show = what_to_show;
        let _filter = ALLOCATOR.g(filter);
        return ALLOCATOR.a(
          _instance.createTreeWalker(_root, _what_to_show, _filter)
        );
      },

      document_create_c_d_a_t_a_section: function(instance, data) {
        let _instance = ALLOCATOR.g(instance);
        let _data = this.s(data);
        return ALLOCATOR.a(_instance.createCDATASection(_data));
      },

      document_create_attribute: function(instance, name) {
        let _instance = ALLOCATOR.g(instance);
        let _name = this.s(name);
        return ALLOCATOR.a(_instance.createAttribute(_name));
      },

      document_create_attribute_n_s: function(instance, namespace, name) {
        let _instance = ALLOCATOR.g(instance);
        let _namespace = this.s(namespace);
        let _name = this.s(name);
        return ALLOCATOR.a(_instance.createAttributeNS(_namespace, _name));
      },

      document_get_location: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.location);
      },

      document_set_location: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.location = ALLOCATOR.g(handle);
      },

      document_get_referrer: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.referrer);
      },

      document_set_referrer: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.referrer = this.s(str);
      },

      document_get_last_modified: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.lastModified);
      },

      document_set_last_modified: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.lastModified = this.s(str);
      },

      document_get_ready_state: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.readyState);
      },

      document_set_ready_state: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.readyState = this.s(str);
      },

      document_get_title: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.title);
      },

      document_set_title: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.title = this.s(str);
      },

      document_get_dir: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.dir);
      },

      document_set_dir: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.dir = this.s(str);
      },

      document_get_body: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.body);
      },

      document_set_body: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.body = ALLOCATOR.g(handle);
      },

      document_get_head: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.head);
      },

      document_set_head: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.head = ALLOCATOR.g(handle);
      },

      document_get_images: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.images);
      },

      document_set_images: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.images = ALLOCATOR.g(handle);
      },

      document_get_embeds: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.embeds);
      },

      document_set_embeds: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.embeds = ALLOCATOR.g(handle);
      },

      document_get_plugins: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.plugins);
      },

      document_set_plugins: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.plugins = ALLOCATOR.g(handle);
      },

      document_get_links: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.links);
      },

      document_set_links: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.links = ALLOCATOR.g(handle);
      },

      document_get_forms: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.forms);
      },

      document_set_forms: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.forms = ALLOCATOR.g(handle);
      },

      document_get_scripts: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.scripts);
      },

      document_set_scripts: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.scripts = ALLOCATOR.g(handle);
      },

      document_get_elements_by_name: function(instance, element_name) {
        let _instance = ALLOCATOR.g(instance);
        let _element_name = this.s(element_name);
        return ALLOCATOR.a(_instance.getElementsByName(_element_name));
      },

      document_get_default_view: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.defaultView);
      },

      document_set_default_view: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.defaultView = ALLOCATOR.g(handle);
      },

      document_has_focus: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.hasFocus());
      },

      document_get_onreadystatechange: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onreadystatechange);
      },

      document_set_onreadystatechange: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onreadystatechange = ALLOCATOR.g(handle);
      },

      document_get_onbeforescriptexecute: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onbeforescriptexecute);
      },

      document_set_onbeforescriptexecute: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onbeforescriptexecute = ALLOCATOR.g(handle);
      },

      document_get_onafterscriptexecute: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onafterscriptexecute);
      },

      document_set_onafterscriptexecute: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onafterscriptexecute = ALLOCATOR.g(handle);
      },

      document_get_onselectionchange: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onselectionchange);
      },

      document_set_onselectionchange: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onselectionchange = ALLOCATOR.g(handle);
      },

      document_get_current_script: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.currentScript);
      },

      document_set_current_script: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.currentScript = ALLOCATOR.g(handle);
      },

      document_release_capture: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.releaseCapture();
      },

      document_get_document_uri_object: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.documentURIObject);
      },

      document_set_document_uri_object: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.documentURIObject = ALLOCATOR.g(handle);
      },

      document_get_referrer_policy: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.referrerPolicy;
      },

      document_set_referrer_policy: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.referrerPolicy = val;
      },

      document_get_anchors: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.anchors);
      },

      document_set_anchors: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.anchors = ALLOCATOR.g(handle);
      },

      document_get_applets: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.applets);
      },

      document_set_applets: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.applets = ALLOCATOR.g(handle);
      },

      document_get_fullscreen: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.fullscreen;
      },

      document_set_fullscreen: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.fullscreen = val;
      },

      document_get_fullscreen_enabled: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.fullscreenEnabled;
      },

      document_set_fullscreen_enabled: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.fullscreenEnabled = val;
      },

      document_exit_fullscreen: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.exitFullscreen();
      },

      document_get_onfullscreenchange: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onfullscreenchange);
      },

      document_set_onfullscreenchange: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onfullscreenchange = ALLOCATOR.g(handle);
      },

      document_get_onfullscreenerror: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onfullscreenerror);
      },

      document_set_onfullscreenerror: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onfullscreenerror = ALLOCATOR.g(handle);
      },

      document_exit_pointer_lock: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.exitPointerLock();
      },

      document_get_onpointerlockchange: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onpointerlockchange);
      },

      document_set_onpointerlockchange: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onpointerlockchange = ALLOCATOR.g(handle);
      },

      document_get_onpointerlockerror: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onpointerlockerror);
      },

      document_set_onpointerlockerror: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onpointerlockerror = ALLOCATOR.g(handle);
      },

      document_get_hidden: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.hidden;
      },

      document_set_hidden: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.hidden = val;
      },

      document_get_visibility_state: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.visibilityState);
      },

      document_set_visibility_state: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.visibilityState = ALLOCATOR.g(handle);
      },

      document_get_onvisibilitychange: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onvisibilitychange);
      },

      document_set_onvisibilitychange: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onvisibilitychange = ALLOCATOR.g(handle);
      },

      document_get_selected_style_sheet_set: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.selectedStyleSheetSet);
      },

      document_set_selected_style_sheet_set: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.selectedStyleSheetSet = this.s(str);
      },

      document_get_last_style_sheet_set: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.lastStyleSheetSet);
      },

      document_set_last_style_sheet_set: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.lastStyleSheetSet = this.s(str);
      },

      document_get_preferred_style_sheet_set: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.preferredStyleSheetSet);
      },

      document_set_preferred_style_sheet_set: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.preferredStyleSheetSet = this.s(str);
      },

      document_get_style_sheet_sets: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.styleSheetSets);
      },

      document_set_style_sheet_sets: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.styleSheetSets = ALLOCATOR.g(handle);
      },

      document_enable_style_sheets_for_set: function(instance, name) {
        let _instance = ALLOCATOR.g(instance);
        let _name = this.s(name);
        _instance.enableStyleSheetsForSet(_name);
      },

      document_caret_position_from_point: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        return ALLOCATOR.a(_instance.caretPositionFromPoint(_x, _y));
      },

      document_get_scrolling_element: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.scrollingElement);
      },

      document_set_scrolling_element: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.scrollingElement = ALLOCATOR.g(handle);
      },

      document_query_selector: function(instance, selectors) {
        let _instance = ALLOCATOR.g(instance);
        let _selectors = this.s(selectors);
        return ALLOCATOR.a(_instance.querySelector(_selectors));
      },

      document_query_selector_all: function(instance, selectors) {
        let _instance = ALLOCATOR.g(instance);
        let _selectors = this.s(selectors);
        return ALLOCATOR.a(_instance.querySelectorAll(_selectors));
      },

      document_get_timeline: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.timeline);
      },

      document_set_timeline: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.timeline = ALLOCATOR.g(handle);
      },

      document_get_animations: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getAnimations());
      },

      document_get_root_element: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.rootElement);
      },

      document_set_root_element: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.rootElement = ALLOCATOR.g(handle);
      },

      document_get_is_srcdoc_document: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.isSrcdocDocument;
      },

      document_set_is_srcdoc_document: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.isSrcdocDocument = val;
      },

      document_get_sandbox_flags_as_string: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.sandboxFlagsAsString);
      },

      document_set_sandbox_flags_as_string: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.sandboxFlagsAsString = this.s(str);
      },

      document_insert_anonymous_content: function(instance, a_element) {
        let _instance = ALLOCATOR.g(instance);
        let _a_element = ALLOCATOR.g(a_element);
        return ALLOCATOR.a(_instance.insertAnonymousContent(_a_element));
      },

      document_remove_anonymous_content: function(instance, a_content) {
        let _instance = ALLOCATOR.g(instance);
        let _a_content = ALLOCATOR.g(a_content);
        _instance.removeAnonymousContent(_a_content);
      },

      document_get_selection: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getSelection());
      },

      document_get_user_has_interacted: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.userHasInteracted;
      },

      document_set_user_has_interacted: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.userHasInteracted = val;
      },

      document_notify_user_gesture_activation: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.notifyUserGestureActivation();
      },

      document_get_document_flash_classification: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.documentFlashClassification);
      },

      document_set_document_flash_classification: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.documentFlashClassification = ALLOCATOR.g(handle);
      },

      element_get_namespace_uri: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.namespaceURI);
      },

      element_set_namespace_uri: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.namespaceURI = this.s(str);
      },

      element_get_prefix: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.prefix);
      },

      element_set_prefix: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.prefix = this.s(str);
      },

      element_get_local_name: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.localName);
      },

      element_set_local_name: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.localName = this.s(str);
      },

      element_get_tag_name: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.tagName);
      },

      element_set_tag_name: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.tagName = this.s(str);
      },

      element_get_id: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.id);
      },

      element_set_id: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.id = this.s(str);
      },

      element_get_class_name: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.className);
      },

      element_set_class_name: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.className = this.s(str);
      },

      element_get_class_list: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.classList);
      },

      element_set_class_list: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.classList = ALLOCATOR.g(handle);
      },

      element_get_attributes: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.attributes);
      },

      element_set_attributes: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.attributes = ALLOCATOR.g(handle);
      },

      element_get_attribute_names: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getAttributeNames());
      },

      element_get_attribute: function(instance, name) {
        let _instance = ALLOCATOR.g(instance);
        let _name = this.s(name);
        return this.ms(_instance.getAttribute(_name));
      },

      element_get_attribute_n_s: function(instance, namespace, local_name) {
        let _instance = ALLOCATOR.g(instance);
        let _namespace = this.s(namespace);
        let _local_name = this.s(local_name);
        return this.ms(_instance.getAttributeNS(_namespace, _local_name));
      },

      element_toggle_attribute: function(instance, name, force) {
        let _instance = ALLOCATOR.g(instance);
        let _name = this.s(name);
        let _force = force;
        return ALLOCATOR.a(_instance.toggleAttribute(_name, _force));
      },

      element_set_attribute: function(instance, name, value) {
        let _instance = ALLOCATOR.g(instance);
        let _name = this.s(name);
        let _value = this.s(value);
        _instance.setAttribute(_name, _value);
      },

      element_set_attribute_n_s: function(instance, namespace, name, value) {
        let _instance = ALLOCATOR.g(instance);
        let _namespace = this.s(namespace);
        let _name = this.s(name);
        let _value = this.s(value);
        _instance.setAttributeNS(_namespace, _name, _value);
      },

      element_remove_attribute: function(instance, name) {
        let _instance = ALLOCATOR.g(instance);
        let _name = this.s(name);
        _instance.removeAttribute(_name);
      },

      element_remove_attribute_n_s: function(instance, namespace, local_name) {
        let _instance = ALLOCATOR.g(instance);
        let _namespace = this.s(namespace);
        let _local_name = this.s(local_name);
        _instance.removeAttributeNS(_namespace, _local_name);
      },

      element_has_attribute: function(instance, name) {
        let _instance = ALLOCATOR.g(instance);
        let _name = this.s(name);
        return ALLOCATOR.a(_instance.hasAttribute(_name));
      },

      element_has_attribute_n_s: function(instance, namespace, local_name) {
        let _instance = ALLOCATOR.g(instance);
        let _namespace = this.s(namespace);
        let _local_name = this.s(local_name);
        return ALLOCATOR.a(_instance.hasAttributeNS(_namespace, _local_name));
      },

      element_has_attributes: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.hasAttributes());
      },

      element_closest: function(instance, selector) {
        let _instance = ALLOCATOR.g(instance);
        let _selector = this.s(selector);
        return ALLOCATOR.a(_instance.closest(_selector));
      },

      element_matches: function(instance, selector) {
        let _instance = ALLOCATOR.g(instance);
        let _selector = this.s(selector);
        return ALLOCATOR.a(_instance.matches(_selector));
      },

      element_webkit_matches_selector: function(instance, selector) {
        let _instance = ALLOCATOR.g(instance);
        let _selector = this.s(selector);
        return ALLOCATOR.a(_instance.webkitMatchesSelector(_selector));
      },

      element_get_elements_with_grid: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getElementsWithGrid());
      },

      element_insert_adjacent_element: function(instance, location, element) {
        let _instance = ALLOCATOR.g(instance);
        let _location = this.s(location);
        let _element = ALLOCATOR.g(element);
        return ALLOCATOR.a(_instance.insertAdjacentElement(_location, _element));
      },

      element_insert_adjacent_text: function(instance, location, data) {
        let _instance = ALLOCATOR.g(instance);
        let _location = this.s(location);
        let _data = this.s(data);
        _instance.insertAdjacentText(_location, _data);
      },

      element_get_font_size_inflation: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.fontSizeInflation;
      },

      element_set_font_size_inflation: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.fontSizeInflation = val;
      },

      element_set_pointer_capture: function(instance, pointer_id) {
        let _instance = ALLOCATOR.g(instance);
        let _pointer_id = pointer_id;
        _instance.setPointerCapture(_pointer_id);
      },

      element_release_pointer_capture: function(instance, pointer_id) {
        let _instance = ALLOCATOR.g(instance);
        let _pointer_id = pointer_id;
        _instance.releasePointerCapture(_pointer_id);
      },

      element_has_pointer_capture: function(instance, pointer_id) {
        let _instance = ALLOCATOR.g(instance);
        let _pointer_id = pointer_id;
        return ALLOCATOR.a(_instance.hasPointerCapture(_pointer_id));
      },

      element_set_capture: function(instance, retarget_to_element) {
        let _instance = ALLOCATOR.g(instance);
        let _retarget_to_element = retarget_to_element;
        _instance.setCapture(_retarget_to_element);
      },

      element_release_capture: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.releaseCapture();
      },

      element_set_capture_always: function(instance, retarget_to_element) {
        let _instance = ALLOCATOR.g(instance);
        let _retarget_to_element = retarget_to_element;
        _instance.setCaptureAlways(_retarget_to_element);
      },

      element_get_attribute_node: function(instance, name) {
        let _instance = ALLOCATOR.g(instance);
        let _name = this.s(name);
        return ALLOCATOR.a(_instance.getAttributeNode(_name));
      },

      element_set_attribute_node: function(instance, new_attr) {
        let _instance = ALLOCATOR.g(instance);
        let _new_attr = ALLOCATOR.g(new_attr);
        return ALLOCATOR.a(_instance.setAttributeNode(_new_attr));
      },

      element_remove_attribute_node: function(instance, old_attr) {
        let _instance = ALLOCATOR.g(instance);
        let _old_attr = ALLOCATOR.g(old_attr);
        return ALLOCATOR.a(_instance.removeAttributeNode(_old_attr));
      },

      element_get_attribute_node_n_s: function(
        instance,
        namespace_uri,
        local_name
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _namespace_uri = this.s(namespace_uri);
        let _local_name = this.s(local_name);
        return ALLOCATOR.a(
          _instance.getAttributeNodeNS(_namespace_uri, _local_name)
        );
      },

      element_set_attribute_node_n_s: function(instance, new_attr) {
        let _instance = ALLOCATOR.g(instance);
        let _new_attr = ALLOCATOR.g(new_attr);
        return ALLOCATOR.a(_instance.setAttributeNodeNS(_new_attr));
      },

      element_scroll_by_no_flush: function(instance, dx, dy) {
        let _instance = ALLOCATOR.g(instance);
        let _dx = dx;
        let _dy = dy;
        return ALLOCATOR.a(_instance.scrollByNoFlush(_dx, _dy));
      },

      element_get_as_flex_container: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getAsFlexContainer());
      },

      element_get_grid_fragments: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getGridFragments());
      },

      element_get_transform_to_ancestor: function(instance, ancestor) {
        let _instance = ALLOCATOR.g(instance);
        let _ancestor = ALLOCATOR.g(ancestor);
        return ALLOCATOR.a(_instance.getTransformToAncestor(_ancestor));
      },

      element_get_transform_to_parent: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getTransformToParent());
      },

      element_get_transform_to_viewport: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getTransformToViewport());
      },

      element_get_client_rects: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getClientRects());
      },

      element_get_bounding_client_rect: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getBoundingClientRect());
      },

      element_scroll_into_view: function(instance, arg) {
        let _instance = ALLOCATOR.g(instance);
        let _arg = ALLOCATOR.g(arg);
        _instance.scrollIntoView(_arg);
      },

      element_get_scroll_top: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.scrollTop;
      },

      element_set_scroll_top: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.scrollTop = val;
      },

      element_get_scroll_left: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.scrollLeft;
      },

      element_set_scroll_left: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.scrollLeft = val;
      },

      element_get_scroll_width: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.scrollWidth;
      },

      element_set_scroll_width: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.scrollWidth = val;
      },

      element_get_scroll_height: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.scrollHeight;
      },

      element_set_scroll_height: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.scrollHeight = val;
      },

      element_scroll: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.scroll(_x, _y);
      },

      element_scroll_to: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.scrollTo(_x, _y);
      },

      element_scroll_by: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.scrollBy(_x, _y);
      },

      element_get_client_top: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.clientTop;
      },

      element_set_client_top: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.clientTop = val;
      },

      element_get_client_left: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.clientLeft;
      },

      element_set_client_left: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.clientLeft = val;
      },

      element_get_client_width: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.clientWidth;
      },

      element_set_client_width: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.clientWidth = val;
      },

      element_get_client_height: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.clientHeight;
      },

      element_set_client_height: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.clientHeight = val;
      },

      element_get_inner_html: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.innerHTML);
      },

      element_set_inner_html: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.innerHTML = this.s(str);
      },

      element_get_outer_html: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.outerHTML);
      },

      element_set_outer_html: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.outerHTML = this.s(str);
      },

      element_insert_adjacent_html: function(instance, position, text) {
        let _instance = ALLOCATOR.g(instance);
        let _position = this.s(position);
        let _text = this.s(text);
        _instance.insertAdjacentHTML(_position, _text);
      },

      element_query_selector: function(instance, selectors) {
        let _instance = ALLOCATOR.g(instance);
        let _selectors = this.s(selectors);
        return ALLOCATOR.a(_instance.querySelector(_selectors));
      },

      element_query_selector_all: function(instance, selectors) {
        let _instance = ALLOCATOR.g(instance);
        let _selectors = this.s(selectors);
        return ALLOCATOR.a(_instance.querySelectorAll(_selectors));
      },

      element_get_shadow_root: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.shadowRoot);
      },

      element_set_shadow_root: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.shadowRoot = ALLOCATOR.g(handle);
      },

      element_get_open_or_closed_shadow_root: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.openOrClosedShadowRoot);
      },

      element_set_open_or_closed_shadow_root: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.openOrClosedShadowRoot = ALLOCATOR.g(handle);
      },

      element_get_assigned_slot: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.assignedSlot);
      },

      element_set_assigned_slot: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.assignedSlot = ALLOCATOR.g(handle);
      },

      element_get_slot: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.slot);
      },

      element_set_slot: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.slot = this.s(str);
      },

      element_request_fullscreen: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.requestFullscreen();
      },

      element_request_pointer_lock: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.requestPointerLock();
      },

      eventtarget_add_event_listener: function(instance, event_type, listener) {
        let _instance = ALLOCATOR.g(instance);
        let _event_type = this.s(event_type);
        let _listener = ALLOCATOR.g(listener);
        _instance.addEventListener(_event_type, _listener);
      },

      eventtarget_remove_event_listener: function(
        instance,
        event_type,
        listener
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _event_type = this.s(event_type);
        let _listener = ALLOCATOR.g(listener);
        _instance.removeEventListener(_event_type, _listener);
      },

      eventtarget_dispatch_event: function(instance, event) {
        let _instance = ALLOCATOR.g(instance);
        let _event = ALLOCATOR.g(event);
        return ALLOCATOR.a(_instance.dispatchEvent(_event));
      },

      htmlcanvaselement_get_width: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.width;
      },

      htmlcanvaselement_set_width: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.width = val;
      },

      htmlcanvaselement_get_height: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.height;
      },

      htmlcanvaselement_set_height: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.height = val;
      },

      htmlcanvaselement_get_context: function(instance, context_id) {
        let _instance = ALLOCATOR.g(instance);
        let _context_id = this.s(context_id);
        return ALLOCATOR.a(_instance.getContext(_context_id));
      },

      htmlcanvaselement_to_data_url: function(
        instance,
        data_type,
        encoder_options
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _data_type = this.s(data_type);
        let _encoder_options = encoder_options;
        return this.ms(_instance.toDataURL(_data_type, _encoder_options));
      },

      htmlcanvaselement_to_blob: function(
        instance,
        callback,
        blob_type,
        encoder_options
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _callback = ALLOCATOR.g(callback);
        let _blob_type = this.s(blob_type);
        let _encoder_options = encoder_options;
        _instance.toBlob(_callback, _blob_type, _encoder_options);
      },

      htmlcanvaselement_transfer_control_to_offscreen: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.transferControlToOffscreen());
      },

      htmlinputelement_get_accept: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.accept);
      },

      htmlinputelement_set_accept: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.accept = this.s(str);
      },

      htmlinputelement_get_alt: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.alt);
      },

      htmlinputelement_set_alt: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.alt = this.s(str);
      },

      htmlinputelement_get_autocomplete: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.autocomplete);
      },

      htmlinputelement_set_autocomplete: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.autocomplete = this.s(str);
      },

      htmlinputelement_get_autofocus: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.autofocus;
      },

      htmlinputelement_set_autofocus: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.autofocus = val;
      },

      htmlinputelement_get_default_checked: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.defaultChecked;
      },

      htmlinputelement_set_default_checked: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.defaultChecked = val;
      },

      htmlinputelement_get_checked: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.checked;
      },

      htmlinputelement_set_checked: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.checked = val;
      },

      htmlinputelement_get_disabled: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.disabled;
      },

      htmlinputelement_set_disabled: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.disabled = val;
      },

      htmlinputelement_get_form: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.form);
      },

      htmlinputelement_set_form: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.form = ALLOCATOR.g(handle);
      },

      htmlinputelement_get_files: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.files);
      },

      htmlinputelement_set_files: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.files = ALLOCATOR.g(handle);
      },

      htmlinputelement_get_form_action: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.formAction);
      },

      htmlinputelement_set_form_action: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.formAction = this.s(str);
      },

      htmlinputelement_get_form_enctype: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.formEnctype);
      },

      htmlinputelement_set_form_enctype: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.formEnctype = this.s(str);
      },

      htmlinputelement_get_form_method: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.formMethod);
      },

      htmlinputelement_set_form_method: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.formMethod = this.s(str);
      },

      htmlinputelement_get_form_no_validate: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.formNoValidate;
      },

      htmlinputelement_set_form_no_validate: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.formNoValidate = val;
      },

      htmlinputelement_get_form_target: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.formTarget);
      },

      htmlinputelement_set_form_target: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.formTarget = this.s(str);
      },

      htmlinputelement_get_height: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.height;
      },

      htmlinputelement_set_height: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.height = val;
      },

      htmlinputelement_get_indeterminate: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.indeterminate;
      },

      htmlinputelement_set_indeterminate: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.indeterminate = val;
      },

      htmlinputelement_get_input_mode: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.inputMode);
      },

      htmlinputelement_set_input_mode: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.inputMode = this.s(str);
      },

      htmlinputelement_get_list: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.list);
      },

      htmlinputelement_set_list: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.list = ALLOCATOR.g(handle);
      },

      htmlinputelement_get_max: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.max);
      },

      htmlinputelement_set_max: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.max = this.s(str);
      },

      htmlinputelement_get_max_length: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.maxLength;
      },

      htmlinputelement_set_max_length: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.maxLength = val;
      },

      htmlinputelement_get_min: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.min);
      },

      htmlinputelement_set_min: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.min = this.s(str);
      },

      htmlinputelement_get_min_length: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.minLength;
      },

      htmlinputelement_set_min_length: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.minLength = val;
      },

      htmlinputelement_get_multiple: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.multiple;
      },

      htmlinputelement_set_multiple: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.multiple = val;
      },

      htmlinputelement_get_name: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.name);
      },

      htmlinputelement_set_name: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.name = this.s(str);
      },

      htmlinputelement_get_pattern: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.pattern);
      },

      htmlinputelement_set_pattern: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.pattern = this.s(str);
      },

      htmlinputelement_get_placeholder: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.placeholder);
      },

      htmlinputelement_set_placeholder: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.placeholder = this.s(str);
      },

      htmlinputelement_get_read_only: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.readOnly;
      },

      htmlinputelement_set_read_only: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.readOnly = val;
      },

      htmlinputelement_get_required: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.required;
      },

      htmlinputelement_set_required: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.required = val;
      },

      htmlinputelement_get_size: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.size;
      },

      htmlinputelement_set_size: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.size = val;
      },

      htmlinputelement_get_src: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.src);
      },

      htmlinputelement_set_src: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.src = this.s(str);
      },

      htmlinputelement_get_step: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.step);
      },

      htmlinputelement_set_step: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.step = this.s(str);
      },

      htmlinputelement_get_type: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.type);
      },

      htmlinputelement_set_type: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.type = this.s(str);
      },

      htmlinputelement_get_default_value: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.defaultValue);
      },

      htmlinputelement_set_default_value: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.defaultValue = this.s(str);
      },

      htmlinputelement_get_value: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.value);
      },

      htmlinputelement_set_value: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.value = this.s(str);
      },

      htmlinputelement_get_value_as_date: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.valueAsDate);
      },

      htmlinputelement_set_value_as_date: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.valueAsDate = ALLOCATOR.g(handle);
      },

      htmlinputelement_get_value_as_number: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.valueAsNumber;
      },

      htmlinputelement_set_value_as_number: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.valueAsNumber = val;
      },

      htmlinputelement_get_width: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.width;
      },

      htmlinputelement_set_width: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.width = val;
      },

      htmlinputelement_get_will_validate: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.willValidate;
      },

      htmlinputelement_set_will_validate: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.willValidate = val;
      },

      htmlinputelement_get_validity: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.validity);
      },

      htmlinputelement_set_validity: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.validity = ALLOCATOR.g(handle);
      },

      htmlinputelement_get_validation_message: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.validationMessage);
      },

      htmlinputelement_set_validation_message: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.validationMessage = this.s(str);
      },

      htmlinputelement_check_validity: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.checkValidity());
      },

      htmlinputelement_report_validity: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.reportValidity());
      },

      htmlinputelement_set_custom_validity: function(instance, error) {
        let _instance = ALLOCATOR.g(instance);
        let _error = this.s(error);
        _instance.setCustomValidity(_error);
      },

      htmlinputelement_get_labels: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.labels);
      },

      htmlinputelement_set_labels: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.labels = ALLOCATOR.g(handle);
      },

      htmlinputelement_select: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.select();
      },

      htmlinputelement_get_selection_direction: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.selectionDirection);
      },

      htmlinputelement_set_selection_direction: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.selectionDirection = this.s(str);
      },

      htmlinputelement_set_range_text: function(
        instance,
        replacement,
        start,
        end,
        selection_mode
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _replacement = this.s(replacement);
        let _start = start;
        let _end = end;
        let _selection_mode = ALLOCATOR.g(selection_mode);
        _instance.setRangeText(_replacement, _start, _end, _selection_mode);
      },

      htmlinputelement_set_selection_range: function(
        instance,
        start,
        end,
        direction
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _start = start;
        let _end = end;
        let _direction = this.s(direction);
        _instance.setSelectionRange(_start, _end, _direction);
      },

      htmlinputelement_get_align: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.align);
      },

      htmlinputelement_set_align: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.align = this.s(str);
      },

      htmlinputelement_get_use_map: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.useMap);
      },

      htmlinputelement_set_use_map: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.useMap = this.s(str);
      },

      htmlinputelement_get_date_time_input_box_value: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getDateTimeInputBoxValue());
      },

      htmlinputelement_update_date_time_input_box: function(instance, value) {
        let _instance = ALLOCATOR.g(instance);
        let _value = ALLOCATOR.g(value);
        _instance.updateDateTimeInputBox(_value);
      },

      htmlinputelement_set_date_time_picker_state: function(instance, open) {
        let _instance = ALLOCATOR.g(instance);
        let _open = open;
        _instance.setDateTimePickerState(_open);
      },

      htmlinputelement_get_minimum: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getMinimum());
      },

      htmlinputelement_get_maximum: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getMaximum());
      },

      htmlinputelement_get_preview_value: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.previewValue);
      },

      htmlinputelement_set_preview_value: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.previewValue = this.s(str);
      },

      keyboardevent_get_char_code: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.charCode;
      },

      keyboardevent_set_char_code: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.charCode = val;
      },

      keyboardevent_get_key_code: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.keyCode;
      },

      keyboardevent_set_key_code: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.keyCode = val;
      },

      keyboardevent_get_alt_key: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.altKey;
      },

      keyboardevent_set_alt_key: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.altKey = val;
      },

      keyboardevent_get_ctrl_key: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.ctrlKey;
      },

      keyboardevent_set_ctrl_key: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.ctrlKey = val;
      },

      keyboardevent_get_shift_key: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.shiftKey;
      },

      keyboardevent_set_shift_key: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.shiftKey = val;
      },

      keyboardevent_get_meta_key: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.metaKey;
      },

      keyboardevent_set_meta_key: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.metaKey = val;
      },

      keyboardevent_get_modifier_state: function(instance, key) {
        let _instance = ALLOCATOR.g(instance);
        let _key = this.s(key);
        return ALLOCATOR.a(_instance.getModifierState(_key));
      },

      keyboardevent_get_location: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.location;
      },

      keyboardevent_set_location: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.location = val;
      },

      keyboardevent_get_repeat: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.repeat;
      },

      keyboardevent_set_repeat: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.repeat = val;
      },

      keyboardevent_get_is_composing: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.isComposing;
      },

      keyboardevent_set_is_composing: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.isComposing = val;
      },

      keyboardevent_get_key: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.key);
      },

      keyboardevent_set_key: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.key = this.s(str);
      },

      keyboardevent_get_code: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.code);
      },

      keyboardevent_set_code: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.code = this.s(str);
      },

      keyboardevent_init_keyboard_event: function(
        instance,
        type_arg,
        bubbles_arg,
        cancelable_arg,
        view_arg,
        key_arg,
        location_arg,
        ctrl_key,
        alt_key,
        shift_key,
        meta_key
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _type_arg = this.s(type_arg);
        let _bubbles_arg = bubbles_arg;
        let _cancelable_arg = cancelable_arg;
        let _view_arg = ALLOCATOR.g(view_arg);
        let _key_arg = this.s(key_arg);
        let _location_arg = location_arg;
        let _ctrl_key = ctrl_key;
        let _alt_key = alt_key;
        let _shift_key = shift_key;
        let _meta_key = meta_key;
        _instance.initKeyboardEvent(
          _type_arg,
          _bubbles_arg,
          _cancelable_arg,
          _view_arg,
          _key_arg,
          _location_arg,
          _ctrl_key,
          _alt_key,
          _shift_key,
          _meta_key
        );
      },

      keyboardevent_get_init_dict: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.initDict);
      },

      keyboardevent_set_init_dict: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.initDict = ALLOCATOR.g(handle);
      },

      mouseevent_get_screen_x: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.screenX;
      },

      mouseevent_set_screen_x: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.screenX = val;
      },

      mouseevent_get_screen_y: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.screenY;
      },

      mouseevent_set_screen_y: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.screenY = val;
      },

      mouseevent_get_client_x: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.clientX;
      },

      mouseevent_set_client_x: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.clientX = val;
      },

      mouseevent_get_client_y: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.clientY;
      },

      mouseevent_set_client_y: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.clientY = val;
      },

      mouseevent_get_x: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.x;
      },

      mouseevent_set_x: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.x = val;
      },

      mouseevent_get_y: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.y;
      },

      mouseevent_set_y: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.y = val;
      },

      mouseevent_get_offset_x: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.offsetX;
      },

      mouseevent_set_offset_x: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.offsetX = val;
      },

      mouseevent_get_offset_y: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.offsetY;
      },

      mouseevent_set_offset_y: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.offsetY = val;
      },

      mouseevent_get_ctrl_key: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.ctrlKey;
      },

      mouseevent_set_ctrl_key: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.ctrlKey = val;
      },

      mouseevent_get_shift_key: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.shiftKey;
      },

      mouseevent_set_shift_key: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.shiftKey = val;
      },

      mouseevent_get_alt_key: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.altKey;
      },

      mouseevent_set_alt_key: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.altKey = val;
      },

      mouseevent_get_meta_key: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.metaKey;
      },

      mouseevent_set_meta_key: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.metaKey = val;
      },

      mouseevent_get_button: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.button;
      },

      mouseevent_set_button: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.button = val;
      },

      mouseevent_get_buttons: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.buttons;
      },

      mouseevent_set_buttons: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.buttons = val;
      },

      mouseevent_get_related_target: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.relatedTarget);
      },

      mouseevent_set_related_target: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.relatedTarget = ALLOCATOR.g(handle);
      },

      mouseevent_get_region: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.region);
      },

      mouseevent_set_region: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.region = this.s(str);
      },

      mouseevent_get_movement_x: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.movementX;
      },

      mouseevent_set_movement_x: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.movementX = val;
      },

      mouseevent_get_movement_y: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.movementY;
      },

      mouseevent_set_movement_y: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.movementY = val;
      },

      mouseevent_init_mouse_event: function(
        instance,
        type_arg,
        can_bubble_arg,
        cancelable_arg,
        view_arg,
        detail_arg,
        screen_x_arg,
        screen_y_arg,
        client_x_arg,
        client_y_arg,
        ctrl_key_arg,
        alt_key_arg,
        shift_key_arg,
        meta_key_arg,
        button_arg,
        related_target_arg
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _type_arg = this.s(type_arg);
        let _can_bubble_arg = can_bubble_arg;
        let _cancelable_arg = cancelable_arg;
        let _view_arg = ALLOCATOR.g(view_arg);
        let _detail_arg = detail_arg;
        let _screen_x_arg = screen_x_arg;
        let _screen_y_arg = screen_y_arg;
        let _client_x_arg = client_x_arg;
        let _client_y_arg = client_y_arg;
        let _ctrl_key_arg = ctrl_key_arg;
        let _alt_key_arg = alt_key_arg;
        let _shift_key_arg = shift_key_arg;
        let _meta_key_arg = meta_key_arg;
        let _button_arg = button_arg;
        let _related_target_arg = ALLOCATOR.g(related_target_arg);
        _instance.initMouseEvent(
          _type_arg,
          _can_bubble_arg,
          _cancelable_arg,
          _view_arg,
          _detail_arg,
          _screen_x_arg,
          _screen_y_arg,
          _client_x_arg,
          _client_y_arg,
          _ctrl_key_arg,
          _alt_key_arg,
          _shift_key_arg,
          _meta_key_arg,
          _button_arg,
          _related_target_arg
        );
      },

      mouseevent_get_modifier_state: function(instance, key_arg) {
        let _instance = ALLOCATOR.g(instance);
        let _key_arg = this.s(key_arg);
        return ALLOCATOR.a(_instance.getModifierState(_key_arg));
      },

      window_get_window: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.window);
      },

      window_set_window: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.window = ALLOCATOR.g(handle);
      },

      window_get_self: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.self);
      },

      window_set_self: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.self = ALLOCATOR.g(handle);
      },

      window_get_document: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.document);
      },

      window_set_document: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.document = ALLOCATOR.g(handle);
      },

      window_get_name: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.name);
      },

      window_set_name: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.name = this.s(str);
      },

      window_get_location: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.location);
      },

      window_set_location: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.location = ALLOCATOR.g(handle);
      },

      window_get_history: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.history);
      },

      window_set_history: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.history = ALLOCATOR.g(handle);
      },

      window_get_custom_elements: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.customElements);
      },

      window_set_custom_elements: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.customElements = ALLOCATOR.g(handle);
      },

      window_get_locationbar: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.locationbar);
      },

      window_set_locationbar: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.locationbar = ALLOCATOR.g(handle);
      },

      window_get_menubar: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.menubar);
      },

      window_set_menubar: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.menubar = ALLOCATOR.g(handle);
      },

      window_get_personalbar: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.personalbar);
      },

      window_set_personalbar: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.personalbar = ALLOCATOR.g(handle);
      },

      window_get_scrollbars: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.scrollbars);
      },

      window_set_scrollbars: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.scrollbars = ALLOCATOR.g(handle);
      },

      window_get_statusbar: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.statusbar);
      },

      window_set_statusbar: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.statusbar = ALLOCATOR.g(handle);
      },

      window_get_toolbar: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.toolbar);
      },

      window_set_toolbar: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.toolbar = ALLOCATOR.g(handle);
      },

      window_get_status: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return this.ms(_instance.status);
      },

      window_set_status: function(instance, str) {
        let _instance = ALLOCATOR.g(instance);
        _instance.status = this.s(str);
      },

      window_close: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.close();
      },

      window_get_closed: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.closed;
      },

      window_set_closed: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.closed = val;
      },

      window_stop: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.stop();
      },

      window_focus: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.focus();
      },

      window_blur: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.blur();
      },

      window_get_event: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.event;
      },

      window_set_event: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.event = val;
      },

      window_get_frames: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.frames);
      },

      window_set_frames: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.frames = ALLOCATOR.g(handle);
      },

      window_get_length: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.length;
      },

      window_set_length: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.length = val;
      },

      window_get_top: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.top);
      },

      window_set_top: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.top = ALLOCATOR.g(handle);
      },

      window_get_opener: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.opener;
      },

      window_set_opener: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.opener = val;
      },

      window_get_parent: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.parent);
      },

      window_set_parent: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.parent = ALLOCATOR.g(handle);
      },

      window_get_frame_element: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.frameElement);
      },

      window_set_frame_element: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.frameElement = ALLOCATOR.g(handle);
      },

      window_open: function(instance, url, target, features) {
        let _instance = ALLOCATOR.g(instance);
        let _url = this.s(url);
        let _target = this.s(target);
        let _features = this.s(features);
        return ALLOCATOR.a(_instance.open(_url, _target, _features));
      },

      window_get_navigator: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.navigator);
      },

      window_set_navigator: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.navigator = ALLOCATOR.g(handle);
      },

      window_get_external: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.external);
      },

      window_set_external: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.external = ALLOCATOR.g(handle);
      },

      window_get_application_cache: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.applicationCache);
      },

      window_set_application_cache: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.applicationCache = ALLOCATOR.g(handle);
      },

      window_alert: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        _instance.alert(_message);
      },

      window_confirm: function(instance, message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        return ALLOCATOR.a(_instance.confirm(_message));
      },

      window_prompt: function(instance, message, default_message) {
        let _instance = ALLOCATOR.g(instance);
        let _message = this.s(message);
        let _default_message = this.s(default_message);
        return this.ms(_instance.prompt(_message, _default_message));
      },

      window_print: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.print();
      },

      window_post_message: function(instance, message, target_origin, transfer) {
        let _instance = ALLOCATOR.g(instance);
        let _message = message;
        let _target_origin = this.s(target_origin);
        let _transfer = ALLOCATOR.g(transfer);
        _instance.postMessage(_message, _target_origin, _transfer);
      },

      window_get_onappinstalled: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onappinstalled);
      },

      window_set_onappinstalled: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onappinstalled = ALLOCATOR.g(handle);
      },

      window_capture_events: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.captureEvents();
      },

      window_release_events: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        _instance.releaseEvents();
      },

      window_get_selection: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.getSelection());
      },

      window_get_computed_style: function(instance, elt, pseudo_elt) {
        let _instance = ALLOCATOR.g(instance);
        let _elt = ALLOCATOR.g(elt);
        let _pseudo_elt = this.s(pseudo_elt);
        return ALLOCATOR.a(_instance.getComputedStyle(_elt, _pseudo_elt));
      },

      window_match_media: function(instance, query) {
        let _instance = ALLOCATOR.g(instance);
        let _query = this.s(query);
        return ALLOCATOR.a(_instance.matchMedia(_query));
      },

      window_get_screen: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.screen);
      },

      window_set_screen: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.screen = ALLOCATOR.g(handle);
      },

      window_move_to: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.moveTo(_x, _y);
      },

      window_move_by: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.moveBy(_x, _y);
      },

      window_resize_to: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.resizeTo(_x, _y);
      },

      window_resize_by: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.resizeBy(_x, _y);
      },

      window_get_inner_width: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.innerWidth;
      },

      window_set_inner_width: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.innerWidth = val;
      },

      window_get_inner_height: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.innerHeight;
      },

      window_set_inner_height: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.innerHeight = val;
      },

      window_scroll: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.scroll(_x, _y);
      },

      window_scroll_to: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.scrollTo(_x, _y);
      },

      window_scroll_by: function(instance, x, y) {
        let _instance = ALLOCATOR.g(instance);
        let _x = x;
        let _y = y;
        _instance.scrollBy(_x, _y);
      },

      window_get_scroll_x: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.scrollX;
      },

      window_set_scroll_x: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.scrollX = val;
      },

      window_get_page_x_offset: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.pageXOffset;
      },

      window_set_page_x_offset: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.pageXOffset = val;
      },

      window_get_scroll_y: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.scrollY;
      },

      window_set_scroll_y: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.scrollY = val;
      },

      window_get_page_y_offset: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.pageYOffset;
      },

      window_set_page_y_offset: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.pageYOffset = val;
      },

      window_get_screen_x: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.screenX;
      },

      window_set_screen_x: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.screenX = val;
      },

      window_get_screen_y: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.screenY;
      },

      window_set_screen_y: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.screenY = val;
      },

      window_get_outer_width: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.outerWidth;
      },

      window_set_outer_width: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.outerWidth = val;
      },

      window_get_outer_height: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.outerHeight;
      },

      window_set_outer_height: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.outerHeight = val;
      },

      window_get_device_pixel_ratio: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.devicePixelRatio;
      },

      window_set_device_pixel_ratio: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.devicePixelRatio = val;
      },

      window_request_animation_frame: function(instance, callback) {
        let _instance = ALLOCATOR.g(instance);
        let _callback = ALLOCATOR.g(callback);
        return ALLOCATOR.a(_instance.requestAnimationFrame(_callback));
      },

      window_cancel_animation_frame: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        let _handle = handle;
        _instance.cancelAnimationFrame(_handle);
      },

      window_get_performance: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.performance);
      },

      window_set_performance: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.performance = ALLOCATOR.g(handle);
      },

      window_get_orientation: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return _instance.orientation;
      },

      window_set_orientation: function(instance, val) {
        let _instance = ALLOCATOR.g(instance);
        _instance.orientation = val;
      },

      window_get_onorientationchange: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onorientationchange);
      },

      window_set_onorientationchange: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onorientationchange = ALLOCATOR.g(handle);
      },

      window_get_onvrdisplayconnect: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onvrdisplayconnect);
      },

      window_set_onvrdisplayconnect: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onvrdisplayconnect = ALLOCATOR.g(handle);
      },

      window_get_onvrdisplaydisconnect: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onvrdisplaydisconnect);
      },

      window_set_onvrdisplaydisconnect: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onvrdisplaydisconnect = ALLOCATOR.g(handle);
      },

      window_get_onvrdisplayactivate: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onvrdisplayactivate);
      },

      window_set_onvrdisplayactivate: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onvrdisplayactivate = ALLOCATOR.g(handle);
      },

      window_get_onvrdisplaydeactivate: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onvrdisplaydeactivate);
      },

      window_set_onvrdisplaydeactivate: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onvrdisplaydeactivate = ALLOCATOR.g(handle);
      },

      window_get_onvrdisplaypresentchange: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.onvrdisplaypresentchange);
      },

      window_set_onvrdisplaypresentchange: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.onvrdisplaypresentchange = ALLOCATOR.g(handle);
      },

      window_get_paint_worklet: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.paintWorklet);
      },

      window_set_paint_worklet: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.paintWorklet = ALLOCATOR.g(handle);
      },

      window_request_idle_callback: function(instance, callback, options) {
        let _instance = ALLOCATOR.g(instance);
        let _callback = ALLOCATOR.g(callback);
        let _options = ALLOCATOR.g(options);
        return ALLOCATOR.a(_instance.requestIdleCallback(_callback, _options));
      },

      window_cancel_idle_callback: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        let _handle = handle;
        _instance.cancelIdleCallback(_handle);
      },

      window_get_origin: function(instance) {
        let _instance = ALLOCATOR.g(instance);
        return ALLOCATOR.a(_instance.origin);
      },

      window_set_origin: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        _instance.origin = ALLOCATOR.g(handle);
      },

      window_btoa: function(instance, btoa) {
        let _instance = ALLOCATOR.g(instance);
        let _btoa = this.s(btoa);
        return this.ms(_instance.btoa(_btoa));
      },

      window_atob: function(instance, atob) {
        let _instance = ALLOCATOR.g(instance);
        let _atob = this.s(atob);
        return this.ms(_instance.atob(_atob));
      },

      window_set_timeout: function(instance, handler, timeout) {
        let _instance = ALLOCATOR.g(instance);
        let _handler = ALLOCATOR.g(handler);
        let _timeout = timeout;
        return ALLOCATOR.a(_instance.setTimeout(_handler, _timeout));
      },

      window_clear_timeout: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        let _handle = handle;
        _instance.clearTimeout(_handle);
      },

      window_set_interval: function(instance, handler, timeout) {
        let _instance = ALLOCATOR.g(instance);
        let _handler = ALLOCATOR.g(handler);
        let _timeout = timeout;
        return ALLOCATOR.a(_instance.setInterval(_handler, _timeout));
      },

      window_clear_interval: function(instance, handle) {
        let _instance = ALLOCATOR.g(instance);
        let _handle = handle;
        _instance.clearInterval(_handle);
      },

      window_create_image_bitmap: function(
        instance,
        a_image,
        a_sx,
        a_sy,
        a_sw,
        a_sh
      ) {
        let _instance = ALLOCATOR.g(instance);
        let _a_image = ALLOCATOR.g(a_image);
        let _a_sx = a_sx;
        let _a_sy = a_sy;
        let _a_sw = a_sw;
        let _a_sh = a_sh;
        return ALLOCATOR.a(
          _instance.createImageBitmap(_a_image, _a_sx, _a_sy, _a_sw, _a_sh)
        );
      }
    };
    return webidl;
  }

  class WebIDLLoader extends HTMLElement {
    connectedCallback() {
      this.utf8dec = new TextDecoder("utf-8");
      this.utf8enc = new TextEncoder("utf-8");
      let wasmSrc = this.getAttribute("module");
      if (!wasmSrc) {
        console.error("no wasm module specified for wasm-module");
        return;
      }
      this.callbackHandler = this.getAttribute("callback") || "callback";
      this.mallocHandler = this.getAttribute("malloc") || "malloc";
      let exec = this.getAttribute("execute") || "main";
      let memory = this.getAttribute("memory") || "memory";
      let isWorker = this.getAttribute("worker");
      let workerId = parseInt(this.getAttribute("worker-id") || 0);
      this.workerId = workerId;
      let messageHandler = this.getAttribute("worker-message") || "message";
      let sharedMemory = this.getAttribute("shared-memory") || false;

      if (isWorker) {
        var response = `
      let utf8dec = new TextDecoder("utf-8");
      let memory = null;
      let instance = null;

      function fromCString(start) {
        const data = new Uint8Array(memory.buffer);
        const str = [];
        let i = start;
        while (data[i] !== 0) {
          str.push(data[i]);
          i++;
        }
        return utf8dec.decode(new Uint8Array(str));
      }

      self.onmessage=function(e){
        if(instance){
          let handler = instance.exports["${messageHandler}"];
          if(handler){
            if (!instance.exports["${this.mallocHandler}"]) {
              throw new Error(
                "Cannot postMessage to wasm without an implementation of malloc(size:i32)->i32 exposed on exports"
              );
            }
            let bytes = e.data;
            let len = bytes.length;
            let start = instance.exports["${this.mallocHandler}"](len);
            const m = new Uint8Array(memory.buffer);
            m.set(bytes, start);
            handler(start,len)
          }
        } else {
          fetch("${wasmSrc}")
            .then(response => response.arrayBuffer())
            .then(bytes => {
              let env = {
                global_postMessage:function(m,len){
                  const data = new Uint8Array(memory.buffer)
                  postMessage(data.subarray(m,m+len))
                },

                console_debug: function(message_start) {
                  let _message = fromCString(message_start);
                  console.debug(_message);
                },

                console_error: function(message_start) {
                  let _message = fromCString(message_start);
                  console.error(_message);
                },

                console_info: function(message_start) {
                  let _message = fromCString(message_start);
                  console.info(_message);
                },

                console_log: function(message_start) {
                  let _message = fromCString(message_start);
                  console.log(_message);
                },
              };
              return WebAssembly.instantiate(bytes, { env });
            })
            .then(results => {
              memory = results.instance.exports["${memory}"];
              instance = results.instance;
              results.instance.exports["${exec}"](${workerId});
              postMessage({type:"load",id:${workerId}});
            });
        }
      }`;
        var blob;
        try {
          blob = new Blob([response], { type: "application/javascript" });
        } catch (e) {
          window.BlobBuilder =
            window.BlobBuilder ||
            window.WebKitBlobBuilder ||
            window.MozBlobBuilder;
          blob = new BlobBuilder();
          blob.append(response);
          blob = blob.getBlob();
        }
        var worker = new Worker(URL.createObjectURL(blob));
        worker.onmessage = e => {
          if (!Array.isArray(e.data) && e.data.type == "load") {
            this.dispatchEvent(new CustomEvent("load", { detail: e.data }));
            this.loaded = true;
            return;
          }
          this.dispatchEvent(new CustomEvent("message", { detail: e.data }));
        };
        this.sendMessage = function(data) {
          worker.postMessage(data);
        };

        // start worker with a message
        worker.postMessage("start");
        return;
      }

      fetch(wasmSrc)
        .then(response => response.arrayBuffer())
        .then(bytes => {
          let webidlContext = createWebIDLContext();
          let env = {};
          let i;
          for (i in webidlContext) {
            env[i] = webidlContext[i].bind(this);
          }
          if (sharedMemory) {
            throw new Error("Not supported yet");
          } else {
            return WebAssembly.instantiate(bytes, { env });
          }
        })
        .then(results => {
          this.memory = results.instance.exports[memory];
          this.exports = results.instance.exports;
          results.instance.exports[exec]();
          this.dispatchEvent(new CustomEvent("load"));
          this.loaded = true;
        })
        .catch(e => {
          console.error(e);
        });
    }

    executeCallback(handle, ev, allocator) {
      let h = this.exports[this.callbackHandler];
      if (h) {
        if (ev) {
          // give the opportunity for event handler to grab what it needs
          let eventHandle = allocator.a(ev);
          h(handle, eventHandle);
          // then release event
          allocator.r(eventHandle);
        } else {
          h(handle, -1);
        }
      } else {
        throw new Error(
          "cannot call back without implementation of callback(source:i32,callback:i32)"
        );
      }
    }

    //readStringFromMemory
    s(start) {
      const data = new Uint8Array(this.memory.buffer);
      const str = [];
      let i = start;
      while (data[i] !== 0) {
        str.push(data[i]);
        i++;
      }
      return this.utf8dec.decode(new Uint8Array(str));
    }

    //makeString
    ms(str) {
      if (!this.exports.malloc) {
        throw new Error(
          "Cannot return string to wasm with an implementation of malloc(size:i32)->i32 exposed on exports"
        );
      }
      let bytes = this.utf8enc.encode(str + String.fromCharCode(0));
      let len = bytes.length;
      let start = this.exports.malloc(len);
      const memory = new Uint8Array(this.memory.buffer);
      memory.set(bytes, start);
      return start;
    }

    m(len) {
      if (!this.exports.malloc) {
        throw new Error(
          "Cannot call malloc to wasm with an implementation of malloc(size:i32)->i32 exposed on exports"
        );
      }
      let start = this.exports.malloc(len);
      return start;
    }
  }
  window.customElements.define("web-dom", WebIDLLoader);

}));
